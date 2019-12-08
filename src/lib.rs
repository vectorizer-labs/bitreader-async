use async_std::io::Read;

/// BitReader reads data from a byte slice at the granularity of a single bit.
pub struct BitReader<R : Read + std::marker::Unpin> {
    reader : BufReader<R>,
    buffer : BitQueue
}

use std::result;

mod bitqueue;
use bitqueue::BitQueue;

mod truncate;

use async_std::io::{BufReader};

mod byte_reader;
use byte_reader::ReadFromBigEndian;

pub mod error;
use error::BitReaderError;

/// Result type for those BitReader operations that can fail.
pub type Result<T> = result::Result<T, BitReaderError>;

impl<R : Read + std::marker::Unpin> BitReader<R>
{
    pub fn new(reader : R) -> BitReader<R>
    {
        BitReader
        {
            reader : BufReader::new(reader),
            buffer : BitQueue::new()
        }
    }

    pub fn is_aligned(&self) -> bool { self.buffer.is_empty() }

    //should be aligned before this is called
    //Reads a T from the Bitreader and increments position accordingly
    pub fn read_aligned_be<T : ReadFromBigEndian>(&mut self) -> Result<T>
    {
        //check if its aligned to the byte mark
        assert!(self.buffer.is_empty());
        Ok(<T>::read_be(&mut self.reader))
    }

    pub fn read_bits<T>(&mut self, count: usize) -> Result<T> 
    where T : Sized + 
              std::ops::Shl<usize, Output=T> + 
              std::ops::Shr<usize, Output=T> + ,
          usize: truncate::TruncateTo<T>
    {
        if self.buffer.len() < count
        {
            //we need to refil the buffer
            
        }

        Ok(self.buffer.pop::<T>(count))
    }
}