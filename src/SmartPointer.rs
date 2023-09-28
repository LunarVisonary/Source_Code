use std::alloc::{self, Layout};

pub struct Sr<T>(*mut (T, i64));

impl<T> Sr<T> {
    pub fn new(val: T) -> Self {
        let layout = Layout::new::<(T, i64)>();
        unsafe {
            let ptr = alloc::alloc(layout) as *mut (T, i64);
            *ptr = (val, 1);
            Sr(ptr)
        }
    }
    
    pub fn get_mut(&mut self) -> &mut T {
        unsafe {
            &mut(*self.0).0
        }
    }

    pub fn get_immut(&self) -> &T {
        unsafe {
            &(*self.0).0
        }
    }
    
    pub fn clone(&mut self) -> Self {
        unsafe {
            (*self.0).1 += 1;
        }
        Self (self.0)
    }
}

impl<T> Drop for Sr<T> {
    fn drop(&mut self) {
        unsafe {
            (*self.0).1 += -1;
            if (*self.0).1 == 0 {
                let layout = Layout::new::<(T, i64)>();
                println!("oh god {}", (*self.0).1);
                alloc::dealloc(self.0 as *mut u8, layout);
            } 
        }
    }
}
