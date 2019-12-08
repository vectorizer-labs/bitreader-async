use std::mem::size_of;
use super::truncate::TruncateTo;

pub struct BitQueue
{
    inner : usize,
    bit_count: usize
}

impl BitQueue
{
    //Intuition
    //The max size poppable from the Bitqueue is the next lowest size
    //For Example, if the inner queue is a u64 the max size of T is u32
    //if bit_count >= 32 then it just pops the first 32 bits inside
    //if the bitcount < 32 then we can add 32 bits and then try to pop again

    pub fn new() -> BitQueue
    {
        BitQueue
        {
            inner : 0,
            bit_count : 0
        }

    }

    //You should be certain that there is no useful data
    //contained in the bits value beyond count 
    //because it will be lost. In general, its a one way trip
    //for bits so copy out any useful bits before calling push
    pub fn push<T : Sized>(&mut self, bits : T, count : usize)
    where T : Sized + std::ops::BitAnd<usize, Output=usize>
    {
        //panics if you try to add more bits than the queue can hold
        assert!(self.bit_count + count <= size_of::<usize>() * 8);

        let trailing_0s : usize = std::mem::size_of::<usize>() - count;

        //trim bit value to make sure we have the right amouunt of trailing 0s 
        let mut bit_value : usize = bits & std::usize::MAX;
        bit_value = bit_value >> trailing_0s;

        //shift the bits back to self.bit_count
        bit_value = bit_value << (trailing_0s - self.bit_count);

        //add the bits to the buffer
        self.inner = self.inner & bit_value;

        self.bit_count += count;
    }

    //TODO: implement a result so we can refill the buffer if we need morre bits than are availible
    //Right now we assume type usize is larger than type T
    //We also assume that popped types don't cross byte boundaries
    pub fn pop<T>(&mut self, count : usize) -> T
    where T : Sized + 
              std::ops::Shl<usize, Output=T> + 
              std::ops::Shr<usize, Output=T> + ,
          usize: TruncateTo<T>
    {
        assert!(self.bit_count > count);

        let mut result : T = TruncateTo::<T>::truncate(&self.inner);

        let trailing_0s = (size_of::<T>() * 8) - count;

        //trim the bits we weren't supposed to read
        result = result >> trailing_0s;

        //shif back
        result = result << trailing_0s;

        //erase count bits from the queue
        self.inner << count;

        //adjust the bit_count
        self.bit_count -= count;

        return result;
    }

    pub fn is_empty(&self) -> bool
    {
        self.bit_count == 0
    }

    pub fn len(&self) -> usize
    {
        self.bit_count
    }
}

/*
std::ops::Shl<usize, Output=T> + 
              std::ops::Shr<usize, Output=T> + 
              std::ops::BitAnd<T, Output=T>

              where T : Sized + 
              std::ops::Shl<usize, Output=T> + 
              std::ops::Shr<usize, Output=T> + 
              std::ops::BitAnd<T, Output=T>
              */