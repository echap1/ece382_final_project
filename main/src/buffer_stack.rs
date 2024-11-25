use core::mem::MaybeUninit;

pub struct BufferStack<T, const MAX: usize> {
    data: [MaybeUninit<T>; MAX],
    next_idx: usize
}

impl<T, const MAX: usize> BufferStack<T, MAX> {
    pub const fn new() -> Self {
        Self {
            data: unsafe { MaybeUninit::uninit().assume_init() },
            next_idx: 0
        }
    }
    
    pub fn push(&mut self, val: T) {
        if self.next_idx == MAX { return }
        self.data[self.next_idx] = MaybeUninit::new(val);
        self.next_idx += 1;
    }
    
    pub fn pop(&mut self) -> Option<&mut T> {
        if self.next_idx == 0 { None }
        else {
            self.next_idx -= 1;
            Some(unsafe { self.data[self.next_idx].assume_init_mut() })
        }
    }
    
    pub fn peek(&self) -> Option<&T> {
        if self.next_idx == 0 { None }
        else { Some(unsafe { self.data[self.next_idx - 1].assume_init_ref() }) }
    }
}