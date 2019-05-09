use crate::utils::settings::Settings;
use super::memory::MemoryModule;
use super::registers::Register;

pub struct SCOM<'a> {
    settings: &'a Settings,
    memory: MemoryModule,
    registers: Vec<Register>,
}

impl<'a> SCOM<'a> {
    pub fn new(settings: &'a Settings) -> SCOM<'a> {
        let register_names = vec![
            Some("rel"), Some("reu"), None, None,
            None, None, None, None,
            None, None, Some("sbl"), Some("sbu"),
            Some("spl"), Some("spu"), Some("pcl"), Some("pcu")
        ];

        let mut registers: Vec<Register> = Vec::new();

        for (i, name) in register_names.iter().enumerate() {
            registers.push(
                Register::new(i, *name)
            )
        }

        SCOM {
            settings: settings,
            memory: MemoryModule::new(settings.memory_size),
            registers,
        }
    }

    pub fn load_program(&mut self, program: &Vec<u8>) {
        self.memory.insert(program, 0);
    }
}
