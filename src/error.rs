/// Result type for those BitReader operations that can fail.
pub type Result<T> = std::result::Result<T, BitReaderError>;

/// Error enumeration of BitReader errors.
#[derive(Fail, Debug)]
pub enum BitReaderError {
    /// Requested more bits than there are left in the byte slice at the current position.
    #[fail(display = "BitReader: Requested {} bits with only {}-{}/{} bits left (position {})", requested, length, position , length, position)]
    NotEnoughData {
        position: usize,
        length: usize,
        requested: usize,
    },
    /// Requested more bits than the returned variable can hold, for example more than 8 bits when
    /// reading into a u8.
    #[fail(display = "BitReader: Requested {} bits while the type can only hold {} (position {})", requested, allowed, position)]
    TooManyBitsForType {
        position: usize,
        requested: u8,
        allowed: u8,
    },
    #[fail(display = "io::Error : {}", _0)]
    Io(#[cause] std::io::Error)
}

#[cfg(feature = "std")]
impl Error for BitReaderError {
    fn description(&self) -> &str {
        match *self {
            BitReaderError::NotEnoughData {..} => "Requested more bits than the byte slice has left",
            BitReaderError::TooManyBitsForType {..} => "Requested more bits than the requested integer type can hold",
            BitReaderError::IoError(e) => e.description()
        }
    }
}

impl From<std::io::Error> for BitReaderError
{
    fn from(error : std::io::Error) -> BitReaderError
    {
        BitReaderError::Io(error)
    }
}