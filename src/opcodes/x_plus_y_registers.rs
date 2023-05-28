use crate::{emulator::Emulator, opcode::Opcode};

pub struct XPlusYRegisters;

impl Opcode for XPlusYRegisters {
    fn execute(&self, code: u16, emulator: &mut Emulator) -> Result<(), ()> {
        let (first, second, third, fourth) = self.decode(code);
        let (8, _, _, 4) = (first, second, third, fourth) else {
            return Err(());
        };

        let x = emulator.ram.get(second as usize);
        let y = emulator.ram.get(third as usize);

        let (result, overflow) = x.overflowing_add(y);

        if overflow {
            emulator.ram.set(0xF, 1);
        } else {
            emulator.ram.set(0xF, 0);
        }

        emulator.ram.set(second as usize, result);

        Ok(())
    }
}
