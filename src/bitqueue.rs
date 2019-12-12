use std::mem::size_of;
use super::truncate::TruncateTo;

pub struct BitQueue
{
    inner : u128,
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
    pub fn push(&mut self, byte : u8)
    {
        let bit_length = size_of::<u8>() * 8;

        //panics if you try to add more bits than the queue can hold
        assert!(self.bit_count + bit_length <= size_of::<u128>() * 8);

        let mut new_bytes : u128 = byte as u128;

        //shift the new bytes by the current bitcount 
        new_bytes = new_bytes << self.bit_count;

        //add the bits to the buffer
        self.inner = self.inner | new_bytes;

        self.bit_count += bit_length;
    }

    //TODO: implement a result so we can refill the buffer if we need morre bits than are availible
    //Right now we assume type usize is larger than type T
    //We also assume that popped types don't cross byte boundaries
    pub fn pop<T>(&mut self, count : usize) -> T
    where T : Sized + std::fmt::Binary +
              std::ops::Shl<usize, Output=T> + 
              std::ops::Shr<usize, Output=T>,
          u128: TruncateTo<T>
    {
        assert!(self.bit_count >= count);

        let mut result : T = TruncateTo::<T>::truncate(&self.inner);

        let trailing_0s = (size_of::<T>() * 8) - count;

        //trim the bits we weren't supposed to read
        result = result << trailing_0s;

        //shif back
        result = result >> trailing_0s;

        //erase count bits from the queue
        self.inner = self.inner << count;

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

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn basic_push() 
    {
        let mut queue = BitQueue::new();
        
        //manually insert a byte
        queue.bit_count = 8;
        queue.inner = 25u128;

        queue.push(25u8);

        let value : u128 = 0b1100100011001;

        assert_eq!(queue.bit_count, 16);
        assert_eq!(queue.inner, value);
    }

    #[test]
    fn basic_pop()
    {
        let mut queue = BitQueue::new();

        //manually insert a byte
        queue.bit_count = 8;
        queue.inner = 25u128;

        assert_eq!(queue.pop::<u8>(8), 25u8);

        //manually insert a byte
        queue.bit_count = 8;
        queue.inner = 25u128;

        assert_eq!(queue.pop::<u16>(8), 25u16);

        //manually insert a byte
        queue.bit_count = 8;
        queue.inner = 25u128;

        assert_eq!(queue.pop::<u32>(8), 25u32);

        //manually insert a byte
        queue.bit_count = 8;
        queue.inner = 25u128;

        assert_eq!(queue.pop::<usize>(8), 25usize);
    }
}