use core::mem::MaybeUninit;
use core::ops::{Add, AddAssign, Div, SubAssign};

pub struct MovingAverage<T: MovingAverageNumber, const SIZE: usize> {
    data: [MaybeUninit<T>; SIZE],
    next_idx: usize,
    filled: bool,
    sum: T
}

impl<T, const SIZE: usize> MovingAverage<T, SIZE> where T: MovingAverageNumber {
    pub const fn new(initial_sum: T) -> Self {
        Self {
            data: unsafe { MaybeUninit::uninit().assume_init() },
            next_idx: 0,
            filled: false,
            sum: initial_sum
        }
    }
    
    pub fn push_and_report(&mut self, new_data: T) -> T {
        self.sum += new_data;
        
        if self.filled {
            self.sum -= unsafe { self.data[self.next_idx].assume_init() };
        } 
        
        self.data[self.next_idx] = MaybeUninit::new(new_data);
        
        let res = if self.filled {
            self.sum.div_by_usize(SIZE)
        } else {
            self.sum.div_by_usize(self.next_idx)
        };
        
        self.next_idx += 1;
        
        res
    }
}

pub trait MovingAverageNumber: Copy + AddAssign + SubAssign + Div {
    fn div_by_usize(self, other: usize) -> Self;
}

impl MovingAverageNumber for u32 {
    fn div_by_usize(self, other: usize) -> Self {
        self / other as u32
    }
}