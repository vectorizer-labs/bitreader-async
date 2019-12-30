#[macro_use] extern crate failure_derive;

use async_std::io::{BufReader, Read};
use async_std::prelude::*;

use bitqueue::BitQueue;
use byte_reader::{ReadFromBigEndian};
use error::Result;

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

    pub fn byte_count(&self) -> &usize
    {
        &self.byte_count
    }

    pub async fn read_be_bits<T>(&mut self, count: usize) -> Result<T> 
    where T : Sized + std::fmt::Binary +
              std::ops::Shl<usize, Output=T> + 
              std::ops::Shr<usize, Output=T> + ,
          u128: truncate::TruncateTo<T>
    {
        println!("Before buffer-len : {}",self.buffer.len());
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

        println!("After buffer-len : {}", self.buffer.len());

        Ok(self.buffer.pop::<T>(count))
    }

    pub async fn read_bits<T>(&mut self, count: usize) -> Result<T> 
    where T : Sized + std::fmt::Binary +
              std::ops::Shl<usize, Output=T> +
              std::ops::Shr<usize, Output=T> + std::convert::From<u8>, u8 : std::convert::From<T>,
          u128: truncate::TruncateTo<T>
    {
        println!("Before buffer-len : {}",self.buffer.len());
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

        println!("After buffer-len : {}", self.buffer.len());

        let result = self.buffer.pop::<T>(count);

        let le_bits = reverse(u8::from(result));

        println!("LE BITS : {:#b}", le_bits);

        Ok(T::from(le_bits))
    }
}

//Taken from here:
//https://stackoverflow.com/questions/2602823/in-c-c-whats-the-simplest-way-to-reverse-the-order-of-bits-in-a-byte/2603254
fn reverse(b : u8) -> u8
{
    // Reverse the top and bottom nibble then swap them.
    return (lookup[(b&0b1111u8) as usize] << 4) | lookup[(b>>4u8) as usize];
}

const lookup : [u8;16] = [
    0x0, 0x8, 0x4, 0xc, 0x2, 0xa, 0x6, 0xe,
    0x1, 0x9, 0x5, 0xd, 0x3, 0xb, 0x7, 0xf, 
];