use crate::{emulator::Emulator, opcode::Opcode};

pub struct SetXToYMinusXRegisters;

impl Opcode for SetXToYMinusXRegisters {
    fn execute(&self, code: u16, emulator: &mut Emulator) -> Result<(), ()> {
        let (first, second, third, fourth) = self.decode(code);
        let (8, _, _, 7) = (first, second, third, fourth) else {
            return Err(());
        };

        let x = emulator.ram.get(second as usize);
        let y = emulator.ram.get(third as usize);

        let (result, overflow) = y.overflowing_sub(x);

        if overflow {
            emulator.ram.set(0xF, 0);
        } else {
            emulator.ram.set(0xF, 1);
        }

        emulator.ram.set(second as usize, result);

        Ok(())
    }
}
