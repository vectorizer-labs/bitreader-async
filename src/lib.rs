use async_std::io::Read;

/// BitReader reads data from a byte slice at the granularity of a single bit.
pub struct BitReader<R : Read> {
    reader : BufReader<R>,
    buf_byte : usize,
    relative_offset: usize,
}

use std::mem::size_of;
use std::result;
use std::error::Error;

use async_std::io::{BufReader};

mod byte_reader;
use byte_reader::ReadFromBigEndian;

pub mod error;
use error::BitReaderError;

/// Result type for those BitReader operations that can fail.
//pub type Result<T> = result::Result<T, BitReaderError>;
pub type Result<T> = result::Result<T, Box<dyn Error>>;

impl<R : Read> BitReader<R>
{
    pub fn new(reader : R) -> BitReader<R>
    {
        BitReader
        {
            reader : BufReader::new(reader), 
            buf_byte : 0,
            relative_offset : 0
        }

    }

    pub fn is_aligned(&self) -> bool { self.relative_offset == 0 }

    //should be aligned before this is called
    //Reads a T from the Bitreader and increments position accordingly
    pub fn read_aligned_be<T : ReadFromBigEndian>(&mut self) -> Result<T>
    {
        //check if its aligned to the byte mark
        assert_eq!(self.relative_offset, 0);

       <T>::read_be(self.reader)
    }

    pub fn read_bits<T>(&mut self, bit_count: usize) -> Result<T> 
    where T : Sized + ReadFromBigEndian + std::ops::Shl<usize, Output=T> + std::ops::Shr<usize, Output=T> + std::ops::BitAnd<T, Output=T>
    {
        let t_size_in_bits = std::mem::size_of::<T>() * 8;

        let beg_byte : usize = self.position();
        let end_byte = beg_byte + size_of::<T>();

        //find the new relative offset
        let new_relative_offset = (bit_count + self.relative_offset) - t_size_in_bits;

        //read the first slice of the num
        let mut num = T::read_be(&mut self.bytes[beg_byte..end_byte]);

        //if the bits fit inside the value trim the end 
        if bit_count < t_size_in_bits
        {
            let remainder = t_size_in_bits - bit_count;

            //trim
            num = num >> remainder;
            num = num << remainder;
        }

        //take care of any beginning relative offset
        num = num << self.relative_offset;

        //Finally, if the bits don't fit inside the value we read earlier
        //we need to add the remainder
        //WE ASSUME bit_count <= t_size_in_bits
        if new_relative_offset > 0
        {
            //read the second slice of the num
            let mut num_remainder = T::read_be_unchecked(&mut self.bytes[beg_byte+1..end_byte+1]);

            //trim by the already read bit amount
            num_remainder = num_remainder >> (t_size_in_bits - self.relative_offset);

            //add them together
            num = num & num_remainder;
        }

        Ok(num)
        
    }
}

