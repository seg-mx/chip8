use crate::{emulator::Emulator, opcode::Opcode};

pub struct SetXToYRegisters;

impl Opcode for SetXToYRegisters {
    fn execute(&self, code: u16, emulator: &mut Emulator) -> Result<(), ()> {
        let (first, second, third, fourth) = self.decode(code);
        let (8, _, _, 0) = (first, second, third, fourth) else {
            return Err(());
        };

        let y = emulator.ram.get(third as usize);
        emulator.ram.set(second as usize, y);

        Ok(())
    }
}
