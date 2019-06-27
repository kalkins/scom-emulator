use super::IO;

struct MemoryModule {
    mem: Vec<u8>,
}

impl IO for MemoryModule {
    fn new(size: usize) -> MemoryModule {
        MemoryModule {
            mem: vec![0_u8; size],
        }
    }

    fn set(&mut self, addr: usize, value: u8) {
        self.mem[addr] = value;
    }

    fn get(&self, addr: usize) -> u8 {
        self.mem[addr]
    }
}

struct MemoryMapModule {
    start: usize,
    end: usize,
    module: Box<dyn IO>,
}

pub struct MemoryController {
    map: Vec<MemoryMapModule>,
}

impl MemoryController {
    pub fn new() -> MemoryController {
        MemoryController { map: Vec::new() }
    }

    pub fn add_memory_module(&mut self, offset: usize, size: usize) {
        self.map.push({
            MemoryMapModule {
                start: offset,
                end: offset + size,
                module: Box::new(MemoryModule::new(size)),
            }
        });
    }

    fn get_module(&mut self, addr: usize) -> Option<&mut MemoryMapModule> {
        for elem in &mut self.map {
            if addr >= elem.start && addr < elem.end {
                return Some(elem)
            }
        }

        None
    }

    fn internal_write(&mut self, addr: usize, value: u8) {
        match self.get_module(addr) {
            Some(elem) => elem.module.set(addr - elem.start, value),
            None => {},
        }
    }

    fn internal_read(&mut self, addr: usize) -> u8 {
        match self.get_module(addr) {
            Some(elem) => elem.module.get(addr - elem.start),
            None => 0,
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        self.internal_write(addr as usize, value);
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        self.internal_read(addr as usize)
    }

    pub fn block_write(&mut self, data: &Vec<u8>, offset: usize) {
        for (i, b) in data.iter().enumerate() {
            self.internal_write(offset + i, *b);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_module() {
        let size: usize = 5;
        let mut module = MemoryModule::new(size);

        for i in 0..size {
            assert_eq!(0, module.get(i));
            module.set(i, i as u8);
        }

        for i in 0..size {
            assert_eq!(i, module.get(i) as usize);
        }
    }

    #[test]
    fn read_write() {
        let mut mmc = MemoryController::new();
        mmc.add_memory_module(0, 1);

        mmc.write(0, 2);
        assert_eq!(2, mmc.read(0));
    }

    #[test]
    fn block_write() {
        let data = vec![1, 2, 3, 4, 5, 6, 7];
        let size: usize = 10;
        let mut mmc = MemoryController::new();
        mmc.add_memory_module(0, size);

        mmc.block_write(&data, 0);

        for i in 0..size {
            if i < data.len() {
                assert_eq!(data[i], mmc.internal_read(i));
            } else {
                assert_eq!(0, mmc.internal_read(i));
            }
        }

        mmc.block_write(&data, size - data.len());

        for i in 0..size {
            if i >= size - data.len() {
                assert_eq!(data[i - (size - data.len())], mmc.internal_read(i));
            } else if i < data.len() {
                assert_eq!(data[i], mmc.internal_read(i));
            } else {
                assert_eq!(0, mmc.internal_read(i));
            }
        }
    }
}
