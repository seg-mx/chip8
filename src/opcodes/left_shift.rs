use crate::{emulator::Emulator, opcode::Opcode};

pub struct LeftShift;

impl Opcode for LeftShift {
    fn execute(&self, code: u16, emulator: &mut Emulator) -> Result<(), ()> {
        let (first, second, third, fourth) = self.decode(code);
        let (8, _, _, 0xE) = (first, second, third, fourth) else {
            return Err(());
        };

        let x = second as usize;

        let removed_bit = (emulator.ram.get(x) >> 7) & 1;
        emulator.ram.set(x, emulator.ram.get(x) << 1);
        emulator.ram.set(0xF, removed_bit);

        Ok(())
    }
}
