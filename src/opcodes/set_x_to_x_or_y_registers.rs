use crate::{emulator::Emulator, opcode::Opcode};

pub struct SetXToXOrYRegisters;

impl Opcode for SetXToXOrYRegisters {
    fn execute(&self, code: u16, emulator: &mut Emulator) -> Result<(), ()> {
        let (first, second, third, fourth) = self.decode(code);
        let (8, _, _, 1) = (first, second, third, fourth) else {
            return Err(());
        };

        let x = emulator.ram.get(second as usize);
        let y = emulator.ram.get(third as usize);

        emulator.ram.set(second as usize, x | y);

        Ok(())
    }
}
