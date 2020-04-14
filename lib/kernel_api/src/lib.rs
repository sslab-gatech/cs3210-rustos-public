#![feature(asm)]
#![no_std]

use core::fmt;

use shim::io;

#[cfg(feature = "user-space")]
pub mod syscall;

pub type OsResult<T> = core::result::Result<T, OsError>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OsError {
    Unknown = 0,
    Ok = 1,

    NoEntry = 10,
    NoMemory = 20,
    NoVmSpace = 30,
    NoAccess = 40,
    BadAddress = 50,
    FileExists = 60,
    InvalidArgument = 70,

    IoError = 101,
    IoErrorEof = 102,
    IoErrorInvalidData = 103,
    IoErrorInvalidInput = 104,
    IoErrorTimedOut = 105,

    InvalidSocket = 200,
    IllegalSocketOperation = 201,
}

impl core::convert::From<u64> for OsError {
    fn from(e: u64) -> Self {
        match e {
            1 => OsError::Ok,

            10 => OsError::NoEntry,
            20 => OsError::NoMemory,
            30 => OsError::NoVmSpace,
            40 => OsError::NoAccess,
            50 => OsError::BadAddress,
            60 => OsError::FileExists,
            70 => OsError::InvalidArgument,

            101 => OsError::IoError,
            102 => OsError::IoErrorEof,
            103 => OsError::IoErrorInvalidData,
            104 => OsError::IoErrorInvalidInput,

            200 => OsError::InvalidSocket,
            201 => OsError::IllegalSocketOperation,

            _ => OsError::Unknown,
        }
    }
}

impl core::convert::From<io::Error> for OsError {
    fn from(e: io::Error) -> Self {
        match e.kind() {
            io::ErrorKind::UnexpectedEof => OsError::IoErrorEof,
            io::ErrorKind::InvalidData => OsError::IoErrorInvalidData,
            io::ErrorKind::InvalidInput => OsError::IoErrorInvalidInput,
            io::ErrorKind::TimedOut => OsError::IoErrorTimedOut,
            io::ErrorKind::NotFound => OsError::NoEntry,
            _ => OsError::IoError,
        }
    }
}

pub const NR_SLEEP: usize = 1;
pub const NR_TIME: usize = 2;
pub const NR_EXIT: usize = 3;
pub const NR_WRITE: usize = 4;
pub const NR_GETPID: usize = 5;
pub const NR_WRITE_STR: usize = 6;

#[derive(Clone, Copy, Debug)]
pub struct SocketDescriptor(u64);

impl SocketDescriptor {
    pub fn raw(&self) -> u64 {
        self.0
    }
}

#[derive(Debug)]
pub struct SocketStatus {
    pub is_active: bool,
    pub is_listening: bool,
    pub can_send: bool,
    pub can_recv: bool,
}

pub struct IpAddr {
    pub ip: u32,
    pub port: u16,
}

impl IpAddr {
    pub fn new((ip1, ip2, ip3, ip4): (u8, u8, u8, u8), port: u16) -> Self {
        IpAddr {
            ip: u32::from_be_bytes([ip1, ip2, ip3, ip4]),
            port,
        }
    }
}

impl fmt::Debug for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.ip.to_be_bytes();
        write!(
            f,
            "IpAddr({}.{}.{}.{}:{})",
            bytes[0], bytes[1], bytes[2], bytes[3], self.port
        )
    }
}

pub const NR_SOCK_CREATE: usize = 20;
pub const NR_SOCK_STATUS: usize = 21;
pub const NR_SOCK_CONNECT: usize = 22;
pub const NR_SOCK_LISTEN: usize = 23;
pub const NR_SOCK_SEND: usize = 24;
pub const NR_SOCK_RECV: usize = 25;
