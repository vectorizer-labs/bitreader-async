use std::fmt;

/// Error enumeration of BitReader errors.
#[derive(Debug,PartialEq,Copy,Clone)]
pub enum BitReaderError {
    /// Requested more bits than there are left in the byte slice at the current position.
    NotEnoughData {
        position: usize,
        length: usize,
        requested: usize,
    },
    /// Requested more bits than the returned variable can hold, for example more than 8 bits when
    /// reading into a u8.
    TooManyBitsForType {
        position: usize,
        requested: u8,
        allowed: u8,
    }
}

#[cfg(feature = "std")]
impl Error for BitReaderError {
    fn description(&self) -> &str {
        match *self {
            BitReaderError::NotEnoughData {..} => "Requested more bits than the byte slice has left",
            BitReaderError::TooManyBitsForType {..} => "Requested more bits than the requested integer type can hold",
        }
    }
}

impl fmt::Display for BitReaderError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        //self.description().fmt(fmt)
        match *self {
            BitReaderError::NotEnoughData { position, length, requested } => write!(fmt, "BitReader: Requested {} bits with only {}/{} bits left (position {})", requested, length - position, length, position),
            BitReaderError::TooManyBitsForType { position, requested, allowed } => write!(fmt, "BitReader: Requested {} bits while the type can only hold {} (position {})", requested, allowed, position),
        }
    }
}