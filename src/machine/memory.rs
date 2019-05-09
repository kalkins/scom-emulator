pub struct MemoryModule {
    memory: Vec<u8>,
}

impl MemoryModule {
    pub fn new(size: usize) -> MemoryModule {
        MemoryModule {
            memory: vec![0; size],
        }
    }

    pub fn insert(&mut self, data: &Vec<u8>, offset: usize) {
        let start = offset;
        let end = offset + data.len();
        self.memory.splice(start..end, data.iter().cloned());
    }
}
