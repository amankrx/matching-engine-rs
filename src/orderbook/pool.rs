// pool.rs

use crate::orderbook::utils::Ptr;

#[derive(Default)]
pub struct Pool<T> {
    pub allocated: Vec<T>,
    pub free: Vec<Ptr>,
}

impl<T: Default> Pool<T> {
    pub fn new() -> Self {
        Pool {
            allocated: Vec::new(),
            free: Vec::new(),
        }
    }

    pub fn new_with_capacity(size: usize) -> Self {
        Pool {
            allocated: Vec::with_capacity(size),
            free: Vec::new(),
        }
    }

    pub fn alloc(&mut self) -> Ptr {
        if let Some(idx) = self.free.pop() {
            idx
        } else {
            let idx = Ptr(self.allocated.len() as u32);
            self.allocated.push(T::default());
            idx
        }
    }

    pub fn free(&mut self, idx: Ptr) {
        self.free.push(idx);
    }

    pub fn get(&self, idx: Ptr) -> Option<&T> {
        let idx = idx.0 as usize;
        if idx < self.allocated.len() {
            Some(&self.allocated[idx])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, idx: Ptr) -> Option<&mut T> {
        let idx = idx.0 as usize;
        if idx < self.allocated.len() {
            Some(&mut self.allocated[idx])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_alloc_free() {
        let mut pool: Pool<i32> = Pool::new_with_capacity(100);

        let ptr1 = pool.alloc();
        let ptr2 = pool.alloc();

        assert_eq!(ptr1.0, 0);
        assert_eq!(ptr2.0, 1);

        pool.free(ptr1);

        let ptr3 = pool.alloc();
        assert_eq!(ptr3.0, 0);
    }
}
