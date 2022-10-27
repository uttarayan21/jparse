use core::fmt::Debug;
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    NomError(String),
    InvalidMarker(u16),
    #[cfg(feature = "bufreader")]
    NomBufferedError(String),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::NomError(e) => write!(f, "Nom error: {}", e),
            Error::InvalidMarker(e) => write!(f, "Invalid marker: {}", e),
            #[cfg(feature = "bufreader")]
            Error::NomBufferedError(e) => write!(f, "Nom buffered error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl<T: Debug> From<nom::Err<T>> for Error {
    fn from(e: nom::Err<T>) -> Self {
        Self::NomError(format!("{:?}", e))
    }
}

#[cfg(feature = "bufreader")]
impl<T: Debug> From<nom_bufreader::Error<T>> for Error {
    fn from(e: nom_bufreader::Error<T>) -> Self {
        Self::NomBufferedError(format!("{:?}", e))
    }
}
