/// Enum representing how much progress has been made transmitting/receiving.
///
/// A value of this type is passed in to the progress callback supplied to
/// methods like [`Xmodem::transmit_with_progress()`],
/// [`Xmodem::receive_with_progress()`], and [`Xmodem::new_with_progress()`]. It
/// is intended to be used by progress indicators or for debugging purposes.
#[derive(Debug, Copy, Clone)]
pub enum Progress {
    /// Waiting for receiver to send NAK.
    Waiting,
    /// Download/upload has started.
    Started,
    /// Packet `.0` was transmitted/received.
    Packet(u8),
    NAK,
    Unknown,
}

/// Type for progress callbacks.
pub type ProgressFn = fn(Progress);

/// Noop progress callback.
pub fn noop(_: Progress) {  }
