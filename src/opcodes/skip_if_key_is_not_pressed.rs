use crate::{emulator::Emulator, opcode::Opcode};

pub struct SkipIfKeyIsNotPressed;

impl Opcode for SkipIfKeyIsNotPressed {
    fn execute(&self, code: u16, emulator: &mut Emulator) -> Result<(), ()> {
        let (first, second, third, fourth) = self.decode(code);
        let (0xE, _, 0xA, 1) = (first, second, third, fourth) else {
            return Err(());
        };

        let x = emulator.ram.get(second as usize);
        if !emulator.keypad.is_pressed(x as usize) {
            emulator.ram.set_pc(emulator.ram.get_pc() + 2);
        }

        Ok(())
    }
}
