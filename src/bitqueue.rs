use std::mem::size_of;

pub struct BitQueue
{
    inner : u8,
    bit_count: usize
}

impl BitQueue
{
    //Intuition: Broken at the moment and only uses u8
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
        assert_eq!(self.inner, 0);

        self.inner = byte;

        self.bit_count += 8;
    }

    //TODO: implement a result so we can refill the buffer if we need more bits than are availible
    //Right now we assume type usize is larger than type T
    //We also assume that popped types don't cross byte boundaries
    pub fn pop(&mut self, count : usize) -> u8
    {
        let mut result = self.inner.clone();

        let trailing_0s = self.bit_count - count;

        result = result >> trailing_0s;

        //if there's at least two bits then we can shift to set to 0
        if self.bit_count > 1
        {
            self.inner = self.inner << count + (size_of::<u8>() * 8 - self.bit_count);
            self.inner = self.inner >> count + (size_of::<u8>() * 8 - self.bit_count);
        }
        //otherwise we have to clear by setting to 0
        else
        {
            self.inner = 0;
        }
        self.bit_count -= count;

        return result as u8;
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
    fn basic_pop()
    {
        let mut queue = BitQueue::new();

        queue.push(0b11001110);

        //println!("pop : {:#b}",queue.pop(4));
        assert_eq!(0b1100, queue.pop(4));  
        assert_eq!(0b1110, queue.inner, "{:#b} != {:#b}",0b1110, queue.inner,);
    }

    #[test]
    fn advanced_pop()
    {
        let mut queue = BitQueue::new();

        queue.push(0b00010000);

        assert_eq!(0b0001, queue.pop(4));
        assert_eq!(0b0, queue.pop(1));  
        assert_eq!(0b00, queue.pop(2));
        assert_eq!(0b0, queue.pop(1));    
    }
}