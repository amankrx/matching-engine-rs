// pool.rs

use crate::level::{Level, LevelId};

#[derive(Default)]
pub struct LevelPool {
    pub allocated: Vec<Level>,
    pub free: Vec<LevelId>,
}

impl LevelPool {
    pub fn new() -> Self {
        Self {
            allocated: Vec::new(),
            free: Vec::new(),
        }
    }

    pub fn new_with_capacity(size: usize) -> Self {
        Self {
            allocated: Vec::with_capacity(size),
            free: Vec::new(),
        }
    }

    pub fn alloc(&mut self) -> LevelId {
        if let Some(idx) = self.free.pop() {
            idx
        } else {
            let idx = LevelId(self.allocated.len() as u32);
            self.allocated.push(Level::default());
            idx
        }
    }

    pub fn free(&mut self, idx: LevelId) {
        self.free.push(idx);
    }

    pub fn get(&self, idx: LevelId) -> Option<&Level> {
        let idx = idx.0 as usize;
        if idx < self.allocated.len() {
            Some(&self.allocated[idx])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, idx: LevelId) -> Option<&mut Level> {
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
        let mut pool: LevelPool = LevelPool::new();

        let ptr1 = pool.alloc();
        let ptr2 = pool.alloc();

        assert_eq!(ptr1.0, 0);
        assert_eq!(ptr2.0, 1);

        pool.free(ptr1);

        let ptr3 = pool.alloc();
        assert_eq!(ptr3.0, 0);
    }
}
