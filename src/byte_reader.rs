use async_std::io::{Read, BufReader};
use async_std::prelude::*;
use std::mem;
use async_trait::async_trait;
use super::error::Result;

#[async_trait]
pub trait ReadFromBigEndian : Sized
{
    //make sure that input is the same size as self before calling
    async fn read_be<R : Read + std::marker::Unpin + std::marker::Send>(reader: &mut BufReader<R>) -> Result<Self>;
}

pub trait FromBigEndian : Sized
{
    fn from_be(int_bytes : Self) -> Self;
}

pub trait FromLittleEndian : Sized
{
    fn from_le(int_bytes : Self) -> Self;
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

        impl FromBigEndian for $ty {
            #[inline]
            fn from_be(int_bytes : $ty) -> Self
            {
                <$ty>::from_be(int_bytes)
            }
        }
        
        impl FromLittleEndian for $ty {
            #[inline]
            fn from_le(int_bytes : $ty) -> Self
            {
                <$ty>::from_le(int_bytes)
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

#[cfg(test)]
mod tests
{
    use super::super::*;
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
    fn read_bool()
    {
        task::block_on(async {
            let bytes = [0b11001110];

            let underlying_reader : BufReader<&[u8]> = BufReader::new(&bytes);

            let mut reader = BitReader::<BufReader<&[u8]>>::new(underlying_reader);

            let num = reader.read_bits::<u8>(4).await.unwrap();

            println!("num : {}", num);
        });
    }
}