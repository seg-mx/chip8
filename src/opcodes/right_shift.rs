use crate::{emulator::Emulator, opcode::Opcode};

pub struct RightShift;

impl Opcode for RightShift {
    fn execute(&self, code: u16, emulator: &mut Emulator) -> Result<(), ()> {
        let (first, second, third, fourth) = self.decode(code);
        let (8, _, _, 6) = (first, second, third, fourth) else {
            return Err(());
        };

        let x = second as usize;

        let removed_bit = emulator.ram.get(x) & 1;
        emulator.ram.set(x, emulator.ram.get(x) >> 1);
        emulator.ram.set(0xF, removed_bit);

        Ok(())
    }
}
