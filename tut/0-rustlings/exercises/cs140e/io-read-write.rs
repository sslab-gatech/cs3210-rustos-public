// FIXME: Make me compile! Diff budget: 2 lines.

// I AM NOT DONE

use std::io;

struct ReadWrapper<T: io::Read> {
    inner: T,
}

impl io::Read for ReadWrapper<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.inner.read(buf)
    }
}

fn main() {}
