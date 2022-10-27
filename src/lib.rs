mod errors;
mod parser;
mod types;
#[cfg(feature = "bufreader")]
use nom_bufreader::{bufreader::BufReader, Parse};

#[cfg(feature = "bufreader")]
use std::io::{Read, Seek};

use std::path::Path;
pub use types::*;

pub fn jpeg_size(path: impl AsRef<Path>) -> Result<usize, crate::errors::Error> {
    let f = std::fs::read(path)?;
    let (_, len) = Jpeg::skip(f.as_slice())?;
    Ok(len)
}

#[cfg(feature = "bufreader")]
pub fn jpeg_size_buffered<I>(max: usize, input: I) -> Result<usize, crate::errors::Error>
where
    I: Read + Seek,
{
    let mut reader = BufReader::with_capacity(max, input);
    let len = reader.parse(Jpeg::__skip_owned_error)?;
    Ok(len)
}
