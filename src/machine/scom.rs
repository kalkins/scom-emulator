use super::instructions::base::{parse_instruction, instruction_length, Instruction, Status};
use super::io::memory::MemoryController;
use super::registers::Register;
use crate::utils::settings::Settings;

pub struct SCOM<'a> {
    settings: &'a Settings,
    mmc: MemoryController,
    registers: Vec<Register>,
}

impl<'a> SCOM<'a> {
    pub fn new(settings: &'a Settings) -> SCOM<'a> {
        let register_names = vec![
            Some("rel"),
            Some("reu"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some("sbl"),
            Some("sbu"),
            Some("spl"),
            Some("spu"),
            Some("pcl"),
            Some("pcu"),
        ];

        let mut registers: Vec<Register> = Vec::new();

        for (i, name) in register_names.iter().enumerate() {
            registers.push(Register::new(i, *name))
        }

        let mut mmc = MemoryController::new();
        mmc.add_memory_module(0, 0xFB00);

        SCOM {
            settings,
            mmc,
            registers,
        }
    }

    pub fn load_program(&mut self, program: &Vec<u8>) {
        self.mmc.block_write(program, 0);
    }

    pub fn run(&mut self) {
        self.execute(0);
    }

    fn execute(&mut self, addr: u16) {
        self.set_big_value(14, 15, addr);

        loop {
            let instruction = self.get_next_instruction();

            match instruction.execute(self) {
                Status::Halt => {
                    println!("Halting");
                    break
                }
                _ => {},
            }
        }
    }

    fn get_next_byte(&mut self) -> u8 {
        let addr = self.get_big_value_and_increment(14, 15);
        self.mmc.read(addr)
    }

    fn get_next_instruction(&mut self) -> Box<dyn Instruction> {
        let mut bytes = vec![self.get_next_byte()];
        let remaining_bytes = instruction_length(bytes[0]) - 1;

        for _ in 0..remaining_bytes {
            bytes.push(self.get_next_byte());
        }

        match parse_instruction(&bytes) {
            Some(instruction) => instruction,
            None => panic!("Failed to decode instruction"),
        }
    }

    fn set_big_value(&mut self, lower: usize, upper: usize, value: u16) {
        let bytes = value.to_be_bytes();
        self.registers[upper].value = bytes[0];
        self.registers[lower].value = bytes[1];
    }

    fn get_big_value(&self, lower: usize, upper: usize) -> u16 {
        u16::from_be_bytes([self.registers[upper].value, self.registers[lower].value])
    }

    fn get_big_value_and_increment(&mut self, lower: usize, upper: usize) -> u16 {
        let value = self.get_big_value(lower, upper);
        self.set_big_value(lower, upper, value + 1);
        value
    }

    fn get_big_value_and_decrement(&mut self, lower: usize, upper: usize) -> u16 {
        let value = self.get_big_value(lower, upper);
        self.set_big_value(lower, upper, value - 1);
        value
    }
}
