pub const RAM_SIZE: usize = 4096;
pub const ROM_START_ADDRESS: usize = 0x200;
pub const FONTSET_START_ADDRESS: usize = 0x050;
const FONTSET_SIZE: usize = 80;

pub struct Ram {
    bytes: [u8; RAM_SIZE],
    pc: u16,
    i_register: u16,
}

impl Ram {
    pub fn new() -> Self {
        let mut ram = Self {
            bytes: [0; RAM_SIZE],
            pc: ROM_START_ADDRESS as u16,
            i_register: 0,
        };

        const FONTSET: [u8; FONTSET_SIZE] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        ram.bytes[FONTSET_START_ADDRESS..FONTSET_START_ADDRESS + FONTSET_SIZE]
            .copy_from_slice(&FONTSET);

        ram
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

    pub fn set_i_register(&mut self, address: u16) {
        self.i_register = address;
    }

    pub fn get_i_register(&self) -> u16 {
        self.i_register
    }
}
