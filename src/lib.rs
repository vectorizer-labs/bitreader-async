#[macro_use] extern crate failure_derive;

use async_std::io::{BufReader, Read};
use async_std::prelude::*;

use bitqueue::BitQueue;
use byte_reader::ReadFromBigEndian;
use error::Result;

/// BitReader reads data from a byte slice at the granularity of a single bit.
pub struct BitReader<R : Read + std::marker::Unpin + std::marker::Send> {
    reader : BufReader<R>,
    buffer : BitQueue
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
            buffer : BitQueue::new()
        }
    }

    pub fn is_aligned(&self) -> bool { self.buffer.is_empty() }

    pub async fn read_u8_slice_aligned(&mut self, count : usize) -> Result<Vec<u8>>
    {
        let mut result : Vec<u8> = Vec::with_capacity(count);
        self.reader.read_exact(result.as_mut_slice()).await?;

        Ok(result)
    }

    //should be aligned before this is called
    //Reads a T from the Bitreader and increments position accordingly
    pub async fn read_aligned_be<T>(&mut self) -> Result<T>
    where T : Sized + ReadFromBigEndian + std::marker::Unpin + std::marker::Send
    {
        //check if its aligned to the byte mark
        assert!(self.buffer.is_empty());
        Ok(<T>::read_be(&mut self.reader).await?)
    }

    pub async fn read_bits<T>(&mut self, count: usize) -> Result<T> 
    where T : Sized + std::fmt::Binary +
              std::ops::Shl<usize, Output=T> + 
              std::ops::Shr<usize, Output=T> + ,
          u128: truncate::TruncateTo<T>
    {
        if self.buffer.len() < count
        {

            let mut buf : [u8; 1] = [0;1];

            self.reader.read_exact(&mut buf).await?;
            println!("Result : {:#?}", buf.len());

            let new_bytes : u8 = u8::from_ne_bytes(buf);

            //we need to refil the buffer
            self.buffer.push(new_bytes);
        }

        Ok(self.buffer.pop::<T>(count))
    }
}