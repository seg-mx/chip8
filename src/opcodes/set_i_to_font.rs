use crate::{emulator::Emulator, opcode::Opcode, ram::FONTSET_START_ADDRESS};

pub struct SetIToFont;

impl Opcode for SetIToFont {
    fn execute(&self, code: u16, emulator: &mut Emulator) -> Result<(), ()> {
        let (first, second, third, fourth) = self.decode(code);
        let (0xF, _, 2, 9) = (first, second, third, fourth) else {
            return Err(());
        };

        let x = second as usize;
        let character = emulator.ram.get(x);

        let address = (character * 5) + FONTSET_START_ADDRESS as u8;
        emulator.ram.set_i_register(address as u16);

        Ok(())
    }
}
