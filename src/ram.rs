pub const RAM_SIZE: usize = 4096;
pub const ROM_START_ADDRESS: usize = 0x200;

pub struct Ram {
    bytes: [u8; RAM_SIZE],
    pc: u16,
}

impl Ram {
    pub fn new() -> Self {
        Self {
            bytes: [0; RAM_SIZE],
            pc: ROM_START_ADDRESS as u16,
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) -> Result<(), String> {
        if rom.len() + ROM_START_ADDRESS >= RAM_SIZE {
            return Err(format!(
                "Rom was too big, rom size: {}, max size: {}",
                rom.len(),
                RAM_SIZE - ROM_START_ADDRESS
            ));
        }

        self.bytes[ROM_START_ADDRESS..rom.len() + ROM_START_ADDRESS].copy_from_slice(rom);

        Ok(())
    }

    pub fn fetch(&mut self) -> u16 {
        let pc = self.pc as usize;

        let op = ((self.bytes[pc] as u16) << 8) | self.bytes[pc + 1] as u16;

        self.pc += 2;

        op
    }

    pub fn set(&mut self, address: usize, value: u8) {
        self.bytes[address] = value;
    }

    pub fn get(&self, address: usize) -> u8 {
        self.bytes[address]
    }

    pub fn set_pc(&mut self, address: u16) {
        self.pc = address;
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }
}
