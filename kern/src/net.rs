///! Network device that wraps USPi in smoltcp abstraction
pub mod uspi;

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryInto;
use core::fmt;
use core::time::Duration;

use smoltcp::iface::{EthernetInterfaceBuilder, NeighborCache};
use smoltcp::phy::{self, Device, DeviceCapabilities};
use smoltcp::socket::{SocketHandle, SocketRef, TcpSocketBuffer};
use smoltcp::time::Instant;
use smoltcp::wire::{IpAddress, IpCidr};

use crate::mutex::Mutex;
use crate::param::MTU;
use crate::USB;

// We always use owned buffer as internal storage
pub type SocketSet = smoltcp::socket::SocketSet<'static, 'static, 'static>;
pub type TcpSocket = smoltcp::socket::TcpSocket<'static>;
pub type EthernetInterface<T> = smoltcp::iface::EthernetInterface<'static, 'static, 'static, T>;

/// 8-byte aligned `u8` slice.
#[repr(align(8))]
struct FrameBuf([u8; MTU as usize]);

/// A fixed size buffer with length tracking functionality.
pub struct Frame {
    buf: Box<FrameBuf>,
    len: u32,
}

impl fmt::Debug for Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Frame")
            .field("buf", &{ self.buf.as_ref() as *const FrameBuf })
            .field("len", &self.len)
            .finish()
    }
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            buf: Box::new(FrameBuf([0; MTU as usize])),
            len: MTU,
        }
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.buf.0.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.buf.0.as_mut_ptr()
    }

    pub fn len(&self) -> u32 {
        self.len
    }

    pub fn set_len(&mut self, len: u32) {
        assert!(len <= MTU as u32);
        self.len = len;
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.buf.0[..self.len as usize]
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.buf.0[..self.len as usize]
    }
}

#[derive(Debug)]
pub struct UsbEthernet;

impl<'a> Device<'a> for UsbEthernet {
    type RxToken = RxToken;
    type TxToken = TxToken;

    fn capabilities(&self) -> DeviceCapabilities {
        let mut capability = DeviceCapabilities::default();
        capability.max_transmission_unit = MTU as usize;
        capability
    }

    fn receive(&'a mut self) -> Option<(Self::RxToken, Self::TxToken)> {
        let mut frame = Frame::new();
        match USB.recv_frame(&mut frame) {
            Some(_) => {
                let rx = RxToken { frame };
                let tx = TxToken;
                Some((rx, tx))
            }
            _ => None,
        }
    }

    fn transmit(&'a mut self) -> Option<Self::TxToken> {
        Some(TxToken)
    }
}

pub struct RxToken {
    frame: Frame,
}

impl phy::RxToken for RxToken {
    fn consume<R, F>(mut self, _timestamp: Instant, f: F) -> smoltcp::Result<R>
    where
        F: FnOnce(&mut [u8]) -> smoltcp::Result<R>,
    {
        f(self.frame.as_mut_slice())
    }
}

pub struct TxToken;

impl phy::TxToken for TxToken {
    fn consume<R, F>(self, _timestamp: Instant, len: usize, f: F) -> smoltcp::Result<R>
    where
        F: FnOnce(&mut [u8]) -> smoltcp::Result<R>,
    {
        let mut frame = Frame::new();
        frame.set_len(len.try_into().unwrap());
        let result = f(frame.as_mut_slice());
        USB.send_frame(&frame);
        result
    }
}

/// Creates and returns a new ethernet interface using `UsbEthernet` struct.
pub fn create_interface() -> EthernetInterface<UsbEthernet> {
    // Lab 5 2.B
    unimplemented!("create_interface")
}

const PORT_MAP_SIZE: usize = 65536 / 64;

pub struct EthernetDriver {
    /// A set of sockets
    socket_set: SocketSet,
    /// Bitmap to track the port usage
    port_map: [u64; PORT_MAP_SIZE],
    /// Internal ethernet interface
    ethernet: EthernetInterface<UsbEthernet>,
}

impl EthernetDriver {
    /// Creates a fresh ethernet driver.
    fn new() -> EthernetDriver {
        // Lab 5 2.B
        unimplemented!("new")
    }

    /// Polls the ethernet interface.
    /// See also `smoltcp::iface::EthernetInterface::poll()`.
    fn poll(&mut self, timestamp: Instant) {
        // Lab 5 2.B
        unimplemented!("poll")
    }

    /// Returns an advisory wait time to call `poll()` the next time.
    /// See also `smoltcp::iface::EthernetInterface::poll_delay()`.
    fn poll_delay(&mut self, timestamp: Instant) -> Duration {
        // Lab 5 2.B
        unimplemented!("poll_delay")
    }

    /// Marks a port as used. Returns `Some(port)` on success, `None` on failure.
    pub fn mark_port(&mut self, port: u16) -> Option<u16> {
        // Lab 5 2.B
        unimplemented!("mark_port")
    }

    /// Clears used bit of a port. Returns `Some(port)` on success, `None` on failure.
    pub fn erase_port(&mut self, port: u16) -> Option<u16> {
        // Lab 5 2.B
        unimplemented!("erase_port")
    }

    /// Returns the first open port between the ephemeral port range 49152 ~ 65535.
    /// Note that this function does not mark the returned port.
    pub fn get_ephemeral_port(&mut self) -> Option<u16> {
        // Lab 5 2.B
        unimplemented!("get_ephemeral_port")
    }

    /// Finds a socket with a `SocketHandle`.
    pub fn get_socket(&mut self, handle: SocketHandle) -> SocketRef<'_, TcpSocket> {
        self.socket_set.get::<TcpSocket>(handle)
    }

    /// This function creates a new TCP socket, adds it to the internal socket
    /// set, and returns the `SocketHandle` of the new socket.
    pub fn add_socket(&mut self) -> SocketHandle {
        let rx_buffer = TcpSocketBuffer::new(vec![0; 16384]);
        let tx_buffer = TcpSocketBuffer::new(vec![0; 16384]);
        let tcp_socket = TcpSocket::new(rx_buffer, tx_buffer);
        self.socket_set.add(tcp_socket)
    }

    /// Releases a socket from the internal socket set.
    pub fn release(&mut self, handle: SocketHandle) {
        self.socket_set.release(handle);
    }

    /// Prunes the internal socket set.
    pub fn prune(&mut self) {
        self.socket_set.prune();
    }
}

/// A thread-safe wrapper for `EthernetDriver`.
pub struct GlobalEthernetDriver(Mutex<Option<EthernetDriver>>);

impl GlobalEthernetDriver {
    pub const fn uninitialized() -> GlobalEthernetDriver {
        GlobalEthernetDriver(Mutex::new(None))
    }

    pub fn initialize(&self) {
        let mut lock = self.0.lock();
        *lock = Some(EthernetDriver::new());
    }

    pub fn poll(&self, timestamp: Instant) {
        // Lab 5 2.B
        unimplemented!("poll")
    }

    pub fn poll_delay(&self, timestamp: Instant) -> Duration {
        self.0
            .lock()
            .as_mut()
            .expect("Uninitialized EthernetDriver")
            .poll_delay(timestamp)
    }

    pub fn mark_port(&self, port: u16) -> Option<u16> {
        self.0
            .lock()
            .as_mut()
            .expect("Uninitialized EthernetDriver")
            .mark_port(port)
    }

    pub fn get_ephemeral_port(&self) -> Option<u16> {
        self.0
            .lock()
            .as_mut()
            .expect("Uninitialized EthernetDriver")
            .get_ephemeral_port()
    }

    pub fn add_socket(&self) -> SocketHandle {
        self.0
            .lock()
            .as_mut()
            .expect("Uninitialized EthernetDriver")
            .add_socket()
    }

    /// Enters a critical region and execute the provided closure with a mutable
    /// reference to the socket.
    pub fn with_socket<F, R>(&self, handle: SocketHandle, f: F) -> R
    where
        F: FnOnce(&mut SocketRef<'_, TcpSocket>) -> R,
    {
        let mut guard = self.0.lock();
        let mut socket = guard
            .as_mut()
            .expect("Uninitialized EthernetDriver")
            .get_socket(handle);

        f(&mut socket)
    }

    /// Enters a critical region and execute the provided closure with a mutable
    /// reference to the inner ethernet driver.
    pub fn critical<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut EthernetDriver) -> R,
    {
        let mut guard = self.0.lock();
        let mut ethernet = guard.as_mut().expect("Uninitialized EthernetDriver");

        f(&mut ethernet)
    }
}
