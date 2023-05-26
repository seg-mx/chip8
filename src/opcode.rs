use crate::{emulator::Emulator, opcodes::*};

pub trait Opcode {
    fn decode(&self, code: u16) -> (u8, u8, u8, u8) {
        let first = ((code & 0xF000) >> 12) as u8;
        let second = ((code & 0x0F00) >> 8) as u8;
        let third = ((code & 0x00F0) >> 4) as u8;
        let fourth = (code & 0x000F) as u8;

        (first, second, third, fourth)
    }

    fn execute(&self, code: u16, emulator: &mut Emulator) -> Result<(), ()>;
}

pub struct OpcodeManager {
    opcodes: [Box<dyn Opcode>; 10],
}

impl OpcodeManager {
    pub fn new() -> Self {
        Self {
            opcodes: [
                Box::new(ClearScreen),
                Box::new(SetIRegister),
                Box::new(SetVRegister),
                Box::new(Draw),
                Box::new(Add),
                Box::new(Jump),
                Box::new(Call),
                Box::new(XMinusYRegisters),
                Box::new(ReturnSubroutine),
                Box::new(SetXToYRegisters),
            ],
        }
    }

    pub fn execute(&mut self, code: u16, emulator: &mut Emulator) {
        let mut result = Err(());

        for opcode in self.opcodes.iter() {
            if let Ok(_) = opcode.execute(code, emulator) {
                result = Ok(());
                break;
            }
        }

        if result.is_err() {
            unimplemented!("Missing behavior for opcode: 0x{code:0>4X}");
        }
    }
}
