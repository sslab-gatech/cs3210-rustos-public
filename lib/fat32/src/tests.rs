extern crate rand;

use std::fmt::{self, Debug};
use std::io;
use std::io::prelude::*;
use std::io::Cursor;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::mbr;
use crate::traits::*;
use crate::vfat;

use mbr::{MasterBootRecord, PartitionEntry, CHS};
use vfat::{BiosParameterBlock, VFat, VFatHandle};

#[derive(Clone)]
struct StdVFatHandle(Arc<Mutex<VFat<Self>>>);

impl Debug for StdVFatHandle {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "StdVFatHandle")
    }
}

impl VFatHandle for StdVFatHandle {
    fn new(val: VFat<StdVFatHandle>) -> Self {
        StdVFatHandle(Arc::new(Mutex::new(val)))
    }

    fn lock<R>(&self, f: impl FnOnce(&mut VFat<StdVFatHandle>) -> R) -> R {
        f(&mut self.0.lock().expect("all okay"))
    }
}

macro check_size($T:ty, $size:expr) {
    assert_eq!(
        ::std::mem::size_of::<$T>(),
        $size,
        "'{}' does not have the expected size of {}",
        stringify!($T),
        $size
    );
}

macro expect_variant($e:expr, $variant:pat $(if $($cond:tt)*)*) {
    match $e {
        $variant $(if $($cond)*)* => {  },
        o => panic!("expected '{}' but found '{:?}'", stringify!($variant), o)
    }
}

macro resource($name:expr) {{
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../ext/fat32-imgs/", $name);
    match ::std::fs::File::open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!(
                "\nfailed to find assignment 2 resource '{}': {}\n\
                 => perhaps you need to run 'make fetch'?",
                $name, e
            );
            panic!("missing resource");
        }
    }
}}

macro assert_hash_eq($name:expr, $actual:expr, $expected:expr) {
    let (actual, expected) = ($actual, $expected);
    let (actual, expected) = (actual.trim(), expected.trim());
    if actual != expected {
        eprintln!("\nFile system hash failed for {}!\n", $name);
        eprintln!("--------------- EXPECTED ---------------");
        eprintln!("{}", expected);
        eprintln!("---------------- ACTUAL ----------------");
        eprintln!("{}", actual);
        eprintln!("---------------- END ----------------");
        panic!("hash mismatch")
    }
}

macro hash_for($name:expr) {{
    let mut file = resource!(concat!("hashes/", $name));
    let mut string = String::new();
    file.read_to_string(&mut string)
        .expect("read hash to string");
    string
}}

macro vfat_from_resource($name:expr) {
    VFat::<StdVFatHandle>::from(resource!($name)).expect("failed to initialize VFAT from image")
}

#[test]
fn check_mbr_size() {
    check_size!(MasterBootRecord, 512);
    check_size!(PartitionEntry, 16);
    check_size!(CHS, 3);
}

#[test]
fn check_mbr_signature() {
    let mut data = [0u8; 512];
    let e = MasterBootRecord::from(Cursor::new(&mut data[..])).unwrap_err();
    expect_variant!(e, mbr::Error::BadSignature);

    data[510..].copy_from_slice(&[0x55, 0xAA]);
    MasterBootRecord::from(Cursor::new(&mut data[..])).unwrap();
}

#[test]
fn check_mbr_boot_indicator() {
    let mut data = [0u8; 512];
    data[510..].copy_from_slice(&[0x55, 0xAA]);

    for i in 0..4usize {
        data[446 + (i.saturating_sub(1) * 16)] = 0;
        data[446 + (i * 16)] = 0xFF;
        let e = MasterBootRecord::from(Cursor::new(&mut data[..])).unwrap_err();
        expect_variant!(e, mbr::Error::UnknownBootIndicator(p) if p == i as u8);
    }

    data[446 + (3 * 16)] = 0;
    MasterBootRecord::from(Cursor::new(&mut data[..])).unwrap();
}

#[test]
fn test_mbr() {
    let mut mbr = resource!("mbr.img");
    let mut data = [0u8; 512];
    mbr.read_exact(&mut data).expect("read resource data");
    MasterBootRecord::from(Cursor::new(&mut data[..])).expect("valid MBR");
}

#[test]
fn check_ebpb_size() {
    check_size!(BiosParameterBlock, 512);
}

#[test]
fn check_ebpb_signature() {
    let mut data = [0u8; 1024];
    data[510..512].copy_from_slice(&[0x55, 0xAA]);

    let e = BiosParameterBlock::from(Cursor::new(&mut data[..]), 1).unwrap_err();
    expect_variant!(e, vfat::Error::BadSignature);

    BiosParameterBlock::from(Cursor::new(&mut data[..]), 0).unwrap();
}

#[test]
fn test_ebpb() {
    let mut ebpb1 = resource!("ebpb1.img");
    let mut ebpb2 = resource!("ebpb2.img");

    let mut data = [0u8; 1024];
    ebpb1
        .read_exact(&mut data[..512])
        .expect("read resource data");
    ebpb2
        .read_exact(&mut data[512..])
        .expect("read resource data");

    BiosParameterBlock::from(Cursor::new(&mut data[..]), 0).expect("valid EBPB");
    BiosParameterBlock::from(Cursor::new(&mut data[..]), 1).expect("valid EBPB");
}

#[test]
fn check_entry_sizes() {
    check_size!(vfat::dir::VFatRegularDirEntry, 32);
    check_size!(vfat::dir::VFatUnknownDirEntry, 32);
    check_size!(vfat::dir::VFatLfnDirEntry, 32);
    check_size!(vfat::dir::VFatDirEntry, 32);
}

#[test]
fn test_vfat_init() {
    vfat_from_resource!("mock1.fat32.img");
    vfat_from_resource!("mock2.fat32.img");
    vfat_from_resource!("mock3.fat32.img");
    vfat_from_resource!("mock4.fat32.img");
}

fn hash_entry<T: Entry>(hash: &mut String, entry: &T) -> ::std::fmt::Result {
    use std::fmt::Write;

    fn write_bool(to: &mut String, b: bool, c: char) -> ::std::fmt::Result {
        if b {
            write!(to, "{}", c)
        } else {
            write!(to, "-")
        }
    }

    fn write_timestamp<T: Timestamp>(to: &mut String, ts: T) -> ::std::fmt::Result {
        write!(
            to,
            "{:02}/{:02}/{} {:02}:{:02}:{:02} ",
            ts.month(),
            ts.day(),
            ts.year(),
            ts.hour(),
            ts.minute(),
            ts.second()
        )
    }

    write_bool(hash, entry.is_dir(), 'd')?;
    write_bool(hash, entry.is_file(), 'f')?;
    write_bool(hash, entry.metadata().read_only(), 'r')?;
    write_bool(hash, entry.metadata().hidden(), 'h')?;
    write!(hash, "\t")?;

    write_timestamp(hash, entry.metadata().created())?;
    write_timestamp(hash, entry.metadata().modified())?;
    write_timestamp(hash, entry.metadata().accessed())?;
    write!(hash, "\t")?;

    write!(hash, "{}", entry.name())?;
    Ok(())
}

fn hash_dir<T: Dir>(hash: &mut String, dir: T) -> Result<Vec<T::Entry>, ::std::fmt::Error> {
    let mut entries: Vec<_> = dir.entries().expect("entries interator").collect();

    entries.sort_by(|a, b| a.name().cmp(b.name()));
    for (i, entry) in entries.iter().enumerate() {
        if i != 0 {
            hash.push('\n');
        }
        hash_entry(hash, entry)?;
    }

    Ok(entries)
}

fn hash_dir_from<P: AsRef<Path>>(vfat: StdVFatHandle, path: P) -> String {
    let mut hash = String::new();
    hash_dir(&mut hash, vfat.open_dir(path).expect("directory exists")).unwrap();
    hash
}

#[test]
fn test_root_entries() {
    let hash = hash_dir_from(vfat_from_resource!("mock1.fat32.img"), "/");
    assert_hash_eq!("mock 1 root directory", hash, hash_for!("root-entries-1"));

    let hash = hash_dir_from(vfat_from_resource!("mock2.fat32.img"), "/");
    assert_hash_eq!("mock 2 root directory", hash, hash_for!("root-entries-2"));

    let hash = hash_dir_from(vfat_from_resource!("mock3.fat32.img"), "/");
    assert_hash_eq!("mock 3 root directory", hash, hash_for!("root-entries-3"));

    let hash = hash_dir_from(vfat_from_resource!("mock4.fat32.img"), "/");
    assert_hash_eq!("mock 4 root directory", hash, hash_for!("root-entries-4"));
}

fn hash_dir_recursive<P: AsRef<Path>>(
    hash: &mut String,
    vfat: StdVFatHandle,
    path: P,
) -> ::std::fmt::Result {
    use std::fmt::Write;

    let path = path.as_ref();
    let dir = vfat.open_dir(path).expect("directory");

    write!(hash, "{}\n", path.display())?;
    let entries = hash_dir(hash, dir)?;
    if entries.iter().any(|e| e.is_dir()) {
        hash.push_str("\n\n");
    }

    for entry in entries {
        if entry.is_dir() && entry.name() != "." && entry.name() != ".." {
            let path = path.join(entry.name());
            hash_dir_recursive(hash, vfat.clone(), path)?;
        }
    }

    Ok(())
}

fn hash_dir_recursive_from<P: AsRef<Path>>(vfat: StdVFatHandle, path: P) -> String {
    let mut hash = String::new();
    hash_dir_recursive(&mut hash, vfat, path).unwrap();
    hash
}

#[test]
fn test_all_dir_entries() {
    let hash = hash_dir_recursive_from(vfat_from_resource!("mock1.fat32.img"), "/");
    assert_hash_eq!("mock 1 all dir entries", hash, hash_for!("all-entries-1"));

    let hash = hash_dir_recursive_from(vfat_from_resource!("mock2.fat32.img"), "/");
    assert_hash_eq!("mock 2 all dir entries", hash, hash_for!("all-entries-2"));

    let hash = hash_dir_recursive_from(vfat_from_resource!("mock3.fat32.img"), "/");
    assert_hash_eq!("mock 3 all dir entries", hash, hash_for!("all-entries-3"));

    let hash = hash_dir_recursive_from(vfat_from_resource!("mock4.fat32.img"), "/");
    assert_hash_eq!("mock 4 all dir entries", hash, hash_for!("all-entries-4"));
}

fn hash_file<T: File>(hash: &mut String, mut file: T) -> ::std::fmt::Result {
    use crate::tests::rand::distributions::{Range, Sample};
    use std::collections::hash_map::DefaultHasher;
    use std::fmt::Write;
    use std::hash::Hasher;

    let mut rng = rand::thread_rng();
    let mut range = Range::new(128, 8192);
    let mut hasher = DefaultHasher::new();

    let mut bytes_read = 0;
    loop {
        let mut buffer = vec![0; range.sample(&mut rng)];
        match file.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                hasher.write(&buffer[..n]);
                bytes_read += n as u64;
            }
            Err(e) => panic!("failed to read file: {:?}", e),
        }
    }

    assert_eq!(
        bytes_read,
        file.size(),
        "expected to read {} bytes (file size) but read {}",
        file.size(),
        bytes_read
    );

    write!(hash, "{}", hasher.finish())
}

fn hash_files_recursive<P: AsRef<Path>>(
    hash: &mut String,
    vfat: StdVFatHandle,
    path: P,
) -> ::std::fmt::Result {
    let path = path.as_ref();
    let mut entries = vfat
        .open_dir(path)
        .expect("directory")
        .entries()
        .expect("entries interator")
        .collect::<Vec<_>>();

    entries.sort_by(|a, b| a.name().cmp(b.name()));
    for entry in entries {
        let path = path.join(entry.name());
        if entry.is_file() && !entry.name().starts_with(".BC.T") {
            use std::fmt::Write;
            let file = entry.into_file().unwrap();
            if file.size() < (1 << 20) {
                write!(hash, "{}: ", path.display())?;
                hash_file(hash, file).expect("successful hash");
                hash.push('\n');
            }
        } else if entry.is_dir() && entry.name() != "." && entry.name() != ".." {
            hash_files_recursive(hash, vfat.clone(), path)?;
        }
    }

    Ok(())
}

fn hash_files_recursive_from<P: AsRef<Path>>(vfat: StdVFatHandle, path: P) -> String {
    let mut hash = String::new();
    hash_files_recursive(&mut hash, vfat, path).unwrap();
    hash
}

#[test]
fn test_mock1_files_recursive() {
    let hash = hash_files_recursive_from(vfat_from_resource!("mock1.fat32.img"), "/");
    assert_hash_eq!("mock 1 file hashes", hash, hash_for!("files-1"));
}

#[test]
fn test_mock2_files_recursive() {
    let hash = hash_files_recursive_from(vfat_from_resource!("mock2.fat32.img"), "/");
    assert_hash_eq!("mock 2 file hashes", hash, hash_for!("files-2-3-4"));
}

#[test]
fn test_mock3_files_recursive() {
    let hash = hash_files_recursive_from(vfat_from_resource!("mock3.fat32.img"), "/");
    assert_hash_eq!("mock 3 file hashes", hash, hash_for!("files-2-3-4"));
}

#[test]
fn test_mock4_files_recursive() {
    let hash = hash_files_recursive_from(vfat_from_resource!("mock4.fat32.img"), "/");
    assert_hash_eq!("mock 4 file hashes", hash, hash_for!("files-2-3-4"));
}

struct Shuffle<T: BlockDevice> {
    device: T,
    swap_address: u64,
}

// Swap two
impl<T: BlockDevice> Shuffle<T> {
    fn new(device: T, swap_address: u64) -> Self {
        let sector_size = device.sector_size();
        assert_eq!(
            swap_address / sector_size,
            (swap_address + 63) / sector_size
        );

        Shuffle {
            device,
            swap_address,
        }
    }

    fn swap_target_n(&self) -> u64 {
        self.swap_address / self.sector_size()
    }

    fn swap_target_offset(&self) -> u64 {
        self.swap_address % self.sector_size()
    }
}

impl<T: BlockDevice> BlockDevice for Shuffle<T> {
    fn sector_size(&self) -> u64 {
        self.device.sector_size()
    }

    fn read_sector(&mut self, n: u64, buf: &mut [u8]) -> io::Result<usize> {
        let bytes = self.device.read_sector(n, buf)?;
        if n == self.swap_target_n() {
            let offset = self.swap_target_offset() as usize;

            let mut front = [0u8; 32];
            front.copy_from_slice(&buf[offset..offset + 32]);
            let mut rear = [0u8; 32];
            rear.copy_from_slice(&buf[offset + 32..offset + 64]);

            buf[offset..offset + 32].copy_from_slice(&rear);
            buf[offset + 32..offset + 64].copy_from_slice(&front);
        }
        Ok(bytes)
    }

    fn write_sector(&mut self, n: u64, buf: &[u8]) -> io::Result<usize> {
        let len = self.sector_size() as usize;
        let mut new_buf = vec![0; len];
        let buf = if n == self.swap_target_n() {
            let offset = self.swap_target_offset() as usize;

            new_buf.copy_from_slice(&buf[..len]);
            new_buf[offset..offset + 32].copy_from_slice(&buf[offset + 32..offset + 64]);
            new_buf[offset + 32..offset + 64].copy_from_slice(&buf[offset..offset + 32]);

            &new_buf
        } else {
            buf
        };
        self.device.write_sector(n, buf)
    }
}

#[test]
fn shuffle_test() {
    let shuffle = Shuffle::new(resource!("mock1.fat32.img"), 0x896ca0);
    let vfat = VFat::<StdVFatHandle>::from(shuffle).expect("failed to initialize VFAT from image");

    let hash = hash_files_recursive_from(vfat, "/");
    assert_hash_eq!("mock 1 file hashes", hash, hash_for!("files-1"));
}
