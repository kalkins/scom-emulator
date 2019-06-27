pub mod memory;

pub trait IO {
    fn new(size: usize) -> Self
    where
        Self: Sized;
    fn set(&mut self, addr: usize, value: u8);
    fn get(&self, addr: usize) -> u8;
}
