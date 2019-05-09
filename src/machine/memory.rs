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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let data = vec![1, 2, 3, 4, 5, 6, 7];
        let size: usize = 10;
        let mut memory = MemoryModule::new(size);

        memory.insert(&data, 0);

        for i in 0..size {
            if i < data.len() {
                assert_eq!(data[i], memory.memory[i]);
            } else {
                assert_eq!(0, memory.memory[i]);
            }
        }

        memory.insert(&data, size - data.len());

        for i in 0..size {
            if i >= size - data.len() {
                assert_eq!(data[i - (size - data.len())], memory.memory[i]);
            } else if i < data.len() {
                assert_eq!(data[i], memory.memory[i]);
            } else {
                assert_eq!(0, memory.memory[i]);
            }
        }
    }
}
