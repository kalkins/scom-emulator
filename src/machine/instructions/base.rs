use super::super::scom::SCOM;

pub enum Status {
    Ok,
    Halt,
}

pub fn instruction_length(first_byte: u8) -> usize {
    1 + ((first_byte & 0b11000000) << 6) as usize
}

pub fn parse_instruction(bytes: &Vec<u8>) -> Option<Box<dyn Instruction>> {
    match instruction_length(bytes[0]) {
        1 => {
            match bytes[0] & 0b00000111 {
                0 => Some(Box::new(NOOP {})),
                1 => Some(Box::new(HALT {})),
                _ => None,
            }
        },
        _ => None,
    }
}

pub trait Instruction {
    fn execute(&self, scom: &mut SCOM) -> Status;
}

struct NOOP {}

impl Instruction for NOOP {
    fn execute(&self, _: &mut SCOM) -> Status {
        Status::Ok
    }
}

struct HALT {}

impl Instruction for HALT {
    fn execute(&self, _: &mut SCOM) -> Status {
        Status::Halt
    }
}
