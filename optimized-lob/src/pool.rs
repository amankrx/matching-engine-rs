// pool.rs

use crate::level::{Level, LevelId};

#[derive(Default, Clone)]
pub struct LevelPool {
    allocated: Vec<Level>,
    free: Vec<LevelId>,
}

impl LevelPool {
    #[inline]
    pub fn new() -> Self {
        Self {
            allocated: Vec::new(),
            free: Vec::new(),
        }
    }

    #[inline]
    pub fn new_with_capacity(size: usize) -> Self {
        Self {
            allocated: Vec::with_capacity(size),
            free: Vec::new(),
        }
    }

    #[inline]
    pub fn alloc(&mut self) -> LevelId {
        if let Some(idx) = self.free.pop() {
            idx
        } else {
            let idx = LevelId(self.allocated.len() as u32);
            self.allocated.push(Level::default());
            idx
        }
    }

    #[inline]
    pub fn free(&mut self, idx: LevelId) {
        self.free.push(idx);
    }

    #[inline]
    pub fn get(&self, idx: LevelId) -> Option<&Level> {
        let idx = idx.value() as usize;
        if idx < self.allocated.len() {
            Some(&self.allocated[idx])
        } else {
            None
        }
    }

    #[inline]
    pub fn get_mut(&mut self, idx: LevelId) -> Option<&mut Level> {
        let idx = idx.value() as usize;
        if idx < self.allocated.len() {
            Some(&mut self.allocated[idx])
        } else {
            None
        }
    }

    #[inline]
    pub fn set_level(&mut self, idx: LevelId, level: Level) {
        let idx = idx.value() as usize;
        self.allocated[idx] = level
    }
}
