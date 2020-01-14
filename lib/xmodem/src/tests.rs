use super::*;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::io::Cursor;

struct Pipe(Sender<u8>, Receiver<u8>, Vec<u8>);

fn pipe() -> (Pipe, Pipe) {
    let ((tx1, rx1), (tx2, rx2)) = (channel(), channel());
    (Pipe(tx1, rx2, vec![]), Pipe(tx2, rx1, vec![]))
}

impl io::Read for Pipe {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        for i in 0..buf.len() {
            match self.1.recv() {
                Ok(byte) => buf[i] = byte,
                Err(_) => return Ok(i)
            }
        }

        Ok(buf.len())
    }
}

impl io::Write for Pipe {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        buf.iter().for_each(|b| self.2.push(*b));
        for (i, byte) in buf.iter().cloned().enumerate() {
            if let Err(e) = self.0.send(byte) {
                eprintln!("Write error: {}", e);
                return Ok(i);
            }
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn test_loop() {
    let mut input = [0u8; 384];
    for (i, chunk) in input.chunks_mut(128).enumerate() {
        chunk.iter_mut().for_each(|b| *b = i as u8);
    }

    let (tx, rx) = pipe();
    let tx_thread = std::thread::spawn(move || Xmodem::transmit(&input[..], rx));
    let rx_thread = std::thread::spawn(move || {
        let mut output = [0u8; 384];
        Xmodem::receive(tx, &mut output[..]).map(|_| output)
    });

    assert_eq!(tx_thread.join().expect("tx join okay").expect("tx okay"), 384);
    let output = rx_thread.join().expect("rx join okay").expect("rx okay");
    assert_eq!(&input[..], &output[..]);
}

#[test]
fn read_byte() {
    let byte = Xmodem::new(Cursor::new(vec![CAN]))
        .read_byte(false)
        .expect("read a byte");

    assert_eq!(byte, CAN);

    let e = Xmodem::new(Cursor::new(vec![CAN]))
        .read_byte(true)
        .expect_err("abort on CAN");

    assert_eq!(e.kind(), io::ErrorKind::ConnectionAborted);
}

#[test]
fn test_expect_byte() {
    let mut xmodem = Xmodem::new(Cursor::new(vec![1, 1]));
    assert_eq!(xmodem.expect_byte(1, "1").expect("expected"), 1);
    let e = xmodem.expect_byte(2, "1, please").expect_err("expect the unexpected");
    assert_eq!(e.kind(), io::ErrorKind::InvalidData);
}

#[test]
fn test_expect_byte_or_cancel() {
    let mut buffer = vec![2, 0];
    let b = Xmodem::new(Cursor::new(buffer.as_mut_slice()))
        .expect_byte_or_cancel(2, "it's a 2")
        .expect("got a 2");

    assert_eq!(b, 2);
}

#[test]
fn test_expect_can() {
    let mut xmodem = Xmodem::new(Cursor::new(vec![CAN]));
    assert_eq!(xmodem.expect_byte(CAN, "hi").expect("CAN"), CAN);
}

#[test]
fn test_unexpected_can() {
    let e = Xmodem::new(Cursor::new(vec![CAN]))
        .expect_byte(SOH, "want SOH")
        .expect_err("have CAN");

    assert_eq!(e.kind(), io::ErrorKind::ConnectionAborted);
}

#[test]
fn test_cancel_on_unexpected() {
    let mut buffer = vec![CAN, 0];
    let e = Xmodem::new(Cursor::new(buffer.as_mut_slice()))
        .expect_byte_or_cancel(SOH, "want SOH")
        .expect_err("have CAN");

    assert_eq!(e.kind(), io::ErrorKind::ConnectionAborted);
    assert_eq!(buffer[1], CAN);

    let mut buffer = vec![0, 0];
    let e = Xmodem::new(Cursor::new(buffer.as_mut_slice()))
        .expect_byte_or_cancel(SOH, "want SOH")
        .expect_err("have 0");

    assert_eq!(e.kind(), io::ErrorKind::InvalidData);
    assert_eq!(buffer[1], CAN);
}

#[test]
fn test_can_in_packet_and_checksum() {
    let mut input = [0u8; 256];
    input[0] = CAN;

    let (tx, rx) = pipe();
    let tx_thread = std::thread::spawn(move || Xmodem::transmit(&input[..], rx));
    let rx_thread = std::thread::spawn(move || {
        let mut output = [0u8; 256];
        Xmodem::receive(tx, &mut output[..]).map(|_| output)
    });

    assert_eq!(tx_thread.join().expect("tx join okay").expect("tx okay"), 256);
    let output = rx_thread.join().expect("rx join okay").expect("rx okay");
    assert_eq!(&input[..], &output[..]);
}

#[test]
fn test_transmit_reported_bytes() {
    let (input, mut output) = ([0u8; 50], [0u8; 128]);
    let (tx, rx) = pipe();
    let tx_thread = std::thread::spawn(move || Xmodem::transmit(&input[..], rx));
    let rx_thread = std::thread::spawn(move || Xmodem::receive(tx, &mut output[..]));
    assert_eq!(tx_thread.join().expect("tx join okay").expect("tx okay"), 50);
    assert_eq!(rx_thread.join().expect("rx join okay").expect("rx okay"), 128);
}

#[test]
fn test_raw_transmission() {
    let mut input = [0u8; 256];
    let mut output = [0u8; 256];
    (0..256usize).into_iter().enumerate().for_each(|(i, b)| input[i] = b as u8);

    let (mut tx, mut rx) = pipe();
    let tx_thread = std::thread::spawn(move || {
        Xmodem::transmit(&input[..], &mut rx).expect("transmit okay");
        rx.2
    });

    let rx_thread = std::thread::spawn(move || {
        Xmodem::receive(&mut tx, &mut output[..]).expect("receive okay");
        tx.2
    });

    let rx_buf = tx_thread.join().expect("tx join okay");
    let tx_buf = rx_thread.join().expect("rx join okay");

    // check packet 1
    assert_eq!(&rx_buf[0..3], &[SOH, 1, 255 - 1]);
    assert_eq!(&rx_buf[3..(3 + 128)], &input[..128]);
    assert_eq!(rx_buf[131], input[..128].iter().fold(0, |a: u8, b| a.wrapping_add(*b)));

    // check packet 2
    assert_eq!(&rx_buf[132..135], &[SOH, 2, 255 - 2]);
    assert_eq!(&rx_buf[135..(135 + 128)], &input[128..]);
    assert_eq!(rx_buf[263], input[128..].iter().fold(0, |a: u8, b| a.wrapping_add(*b)));

    // check EOT
    assert_eq!(&rx_buf[264..], &[EOT, EOT]);

    // check receiver responses
    assert_eq!(&tx_buf, &[NAK, ACK, ACK, NAK, ACK]);
}

#[test]
fn test_small_packet_eof_error() {
    let mut xmodem = Xmodem::new(Cursor::new(vec![NAK, NAK, NAK]));

    let mut buffer = [1, 2, 3];
    let e = xmodem.read_packet(&mut buffer[..]).expect_err("read EOF");
    assert_eq!(e.kind(), io::ErrorKind::UnexpectedEof);

    let e = xmodem.write_packet(&buffer).expect_err("write EOF");
    assert_eq!(e.kind(), io::ErrorKind::UnexpectedEof);
}

#[test]
fn test_bad_control() {
    let mut packet = [0; 128];
    let e = Xmodem::new(Cursor::new(vec![0, CAN]))
        .read_packet(&mut packet[..])
        .expect_err("CAN");

    assert_eq!(e.kind(), io::ErrorKind::ConnectionAborted);

    let e = Xmodem::new(Cursor::new(vec![0, 0xFF]))
        .read_packet(&mut packet[..])
        .expect_err("bad contorl");

    assert_eq!(e.kind(), io::ErrorKind::InvalidData);
}

#[test]
fn test_eot() {
    let mut buffer = vec![NAK, 0, NAK, 0, ACK];
    Xmodem::new(Cursor::new(buffer.as_mut_slice()))
        .write_packet(&[])
        .expect("write empty buf for EOT");

    assert_eq!(&buffer[..], &[NAK, EOT, NAK, EOT, ACK]);
}
