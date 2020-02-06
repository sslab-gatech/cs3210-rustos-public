use shim::io;

use crate::mbr;

#[derive(Debug)]
pub enum Error {
    Mbr(mbr::Error),
    Io(io::Error),
    BadSignature,
    NotFound,
}

impl From<mbr::Error> for Error {
    fn from(error: mbr::Error) -> Error {
        Error::Mbr(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::Io(error)
    }
}
