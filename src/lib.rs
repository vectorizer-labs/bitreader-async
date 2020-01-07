#[macro_use] 
extern crate failure;

use async_std::io::{BufReader, Read};
use async_std::prelude::*;

use bitqueue::BitQueue;
use byte_reader::{ReadFromBigEndian};
use error::Result;

use bit_reverse::LookupReverse;

/// BitReader reads data from a byte slice at the granularity of a single bit.
pub struct BitReader<R : Read + std::marker::Unpin + std::marker::Send> {
    reader : BufReader<R>,
    buffer : BitQueue,
    byte_count : usize
}

mod bitqueue;
mod truncate;
mod byte_reader;

pub mod error;

impl<R : Read + std::marker::Unpin + std::marker::Send> BitReader<R>
{
    pub fn new(reader : R) -> BitReader<R>
    {
        BitReader
        {
            reader : BufReader::new(reader),
            buffer : BitQueue::new(),
            byte_count : 0usize
        }
    }

    pub fn is_aligned(&self) -> bool { self.buffer.is_empty() }

    pub async fn read_u8_slice_aligned(&mut self, count : usize) -> Result<Vec<u8>>
    {
        let mut result : Vec<u8> = vec![0; count];
        self.reader.read_exact(result.as_mut_slice()).await?;

        self.byte_count += count;

        Ok(result)
    }

    //should be aligned before this is called
    //Reads a T from the Bitreader and increments position accordingly
    pub async fn read_aligned_be<T>(&mut self) -> Result<T>
    where T : Sized + ReadFromBigEndian + std::marker::Unpin + std::marker::Send
    {
        //check if its aligned to the byte mark
        assert!(self.buffer.is_empty());
        let num = <T>::read_be(&mut self.reader).await?;
        self.byte_count += std::mem::size_of::<T>();
        Ok(num)
    }

    pub fn byte_count(&self) -> usize
    {
        self.byte_count.clone()
    }

    pub async fn read_be_bits(&mut self, count: usize) -> Result<u8> 
    {
        if self.buffer.len() < count
        {
            //TODO: expand this to allow for bitstruct types bigger than one byte
            let mut buf : [u8; 1] = [0;1];

            self.reader.read_exact(&mut buf).await?;

            self.byte_count += 1;

            let new_bytes : u8 = u8::from_ne_bytes(buf);

            //we need to refil the buffer
            self.buffer.push(new_bytes);
        }

        Ok(self.buffer.pop(count))
    }

    //TODO: come back and make this generic
    //If some protocol needs it
    pub async fn read_le_bits(&mut self, count: usize) -> Result<u8>
    {
        if self.buffer.len() < count
        {
            //TODO: expand this to allow for bitstruct types bigger than one byte
            let mut buf : [u8; 1] = [0;1];

            self.reader.read_exact(&mut buf).await?;

            self.byte_count += 1;

            let new_bytes : u8 = u8::from_ne_bytes(buf);

            //we need to refil the buffer
            self.buffer.push(new_bytes);
        }

        let trailing_0s = (std::mem::size_of::<u8>() * 8) - count;

        let result = self.buffer.pop(count).swap_bits() >> trailing_0s;

        println!("LE Bits : {:#010b}", result);

        Ok(result)
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use async_std::io::BufReader;
    use async_std::task;

    #[test]
    fn read_be() 
    {
        task::block_on(async {
            let bytes = [25u8,25u8,25u8,25u8];

            let underlying_reader : BufReader<&[u8]> = BufReader::new(&bytes);

            let mut reader = BitReader::<BufReader<&[u8]>>::new(underlying_reader);

            let num = reader.read_aligned_be::<u16>().await.unwrap();

            assert_eq!(num, 6425u16);

            let num2 = reader.read_aligned_be::<u8>().await.unwrap();

            assert_eq!(num2, 25u8);
        });
    }

    #[test]
    fn read_test_connect_fixed_header()
    {
        task::block_on(async {
            let bytes = [0b00010000];

            let underlying_reader : BufReader<&[u8]> = BufReader::new(&bytes);

            let mut reader = BitReader::<BufReader<&[u8]>>::new(underlying_reader);


            assert_eq!(0b0001, reader.read_be_bits(4).await.unwrap());
            assert_eq!(0b0, reader.read_be_bits(1).await.unwrap());
            assert_eq!(0b00, reader.read_be_bits(2).await.unwrap());
            assert_eq!(0b0, reader.read_be_bits(1).await.unwrap());
        });
    }

    #[test]
    fn read_test_connect_flags()
    {
        task::block_on(async {
            let bytes = [0b11001110];

            let underlying_reader : BufReader<&[u8]> = BufReader::new(&bytes);

            let mut reader = BitReader::<BufReader<&[u8]>>::new(underlying_reader);


            assert_eq!(0b1, reader.read_be_bits(1).await.unwrap());
            assert_eq!(0b1, reader.read_be_bits(1).await.unwrap());
            assert_eq!(0b0, reader.read_be_bits(1).await.unwrap());
            assert_eq!(0b01, reader.read_be_bits(2).await.unwrap());
            assert_eq!(0b1, reader.read_be_bits(1).await.unwrap());
            assert_eq!(0b1, reader.read_be_bits(1).await.unwrap());
            assert_eq!(0b0, reader.read_be_bits(1).await.unwrap());
        });
    }
}