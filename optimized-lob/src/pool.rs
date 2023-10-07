// pool.rs

// Import the Level and LevelId structs from the level module.
use crate::level::{Level, LevelId};

// Define a struct named LevelPool, which is a pool for managing Level objects.
#[derive(Default, Clone)]
pub struct LevelPool {
    allocated: Vec<Level>, // A vector to store allocated Level objects.
    free: Vec<LevelId>,    // A vector to store free LevelId values.
}

impl LevelPool {
    // Constructor for creating a new LevelPool instance with default values.
    #[inline]
    pub fn new() -> Self {
        Self {
            allocated: Vec::new(), // Initialize allocated vector as empty.
            free: Vec::new(),      // Initialize free vector as empty.
        }
    }

    // Constructor for creating a new LevelPool instance with a specified capacity.
    #[inline]
    pub fn new_with_capacity(size: usize) -> Self {
        Self {
            allocated: Vec::with_capacity(size), // Initialize allocated vector with the specified capacity.
            free: Vec::new(),                    // Initialize free vector as empty.
        }
    }

    // Allocate a LevelId from the pool. Reuses a free LevelId if available or creates a new one.
    #[inline]
    pub fn alloc(&mut self) -> LevelId {
        if let Some(idx) = self.free.pop() {
            idx // Reuse a free LevelId.
        } else {
            let idx = LevelId(self.allocated.len() as u32); // Create a new LevelId.
            self.allocated.push(Level::default()); // Allocate a new Level object.
            idx
        }
    }

    // Free a LevelId by adding it back to the pool of available LevelIds.
    #[inline]
    pub fn free(&mut self, idx: LevelId) {
        self.free.push(idx);
    }

    // Get a reference to a Level by LevelId if it exists in the pool.
    #[inline]
    pub fn get(&self, idx: LevelId) -> Option<&Level> {
        let idx = idx.value() as usize;
        if idx < self.allocated.len() {
            Some(&self.allocated[idx])
        } else {
            None
        }
    }

    // Get a mutable reference to a Level by LevelId if it exists in the pool.
    #[inline]
    pub fn get_mut(&mut self, idx: LevelId) -> Option<&mut Level> {
        let idx = idx.value() as usize;
        if idx < self.allocated.len() {
            Some(&mut self.allocated[idx])
        } else {
            None
        }
    }

    // Set the Level object associated with a LevelId in the pool.
    #[inline]
    pub fn set_level(&mut self, idx: LevelId, level: Level) {
        let idx = idx.value() as usize;
        self.allocated[idx] = level
    }
}
