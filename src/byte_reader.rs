use async_std::io::{Read, BufReader};
use async_std::prelude::*;
use std::mem;
use async_trait::async_trait;
use super::error::Result;


//TODO: make this async so its actually fast lmao
#[async_trait]
pub trait ReadFromBigEndian : Sized
{
    //make sure that input is the same size as self before calling
    async fn read_be<R : Read + std::marker::Unpin + std::marker::Send>(reader: &mut BufReader<R>) -> Result<Self>;
}

macro_rules! be_impl {
    ($ty: ty, $size: tt) => {
        #[async_trait]
        impl ReadFromBigEndian for $ty {
            #[inline]
            async fn read_be<R : Read + std::marker::Unpin + std::marker::Send>(reader: &mut BufReader<R>) -> Result<Self>
            {
                let mut int_bytes = [0u8; $size];
                let _result = reader.read_exact(&mut int_bytes).await?;
                Ok(<$ty>::from_be_bytes(int_bytes))
            }
        }
    }
}

be_impl!(u8, (mem::size_of::<u8>()));
be_impl!(u16, (mem::size_of::<u16>()));
be_impl!(u32, (mem::size_of::<u32>()));
be_impl!(u64, (mem::size_of::<u64>()));
be_impl!(i8,  (mem::size_of::<i8>()));
be_impl!(i16, (mem::size_of::<i16>()));
be_impl!(i32, (mem::size_of::<i32>()));
be_impl!(i64, (mem::size_of::<i64>()));
be_impl!(usize, (mem::size_of::<usize>()));
be_impl!(isize, (mem::size_of::<isize>()));