use crate::{emulator::Emulator, opcode::Opcode};

pub struct SkipIfXNotEqualsY;

impl Opcode for SkipIfXNotEqualsY {
    fn execute(&self, code: u16, emulator: &mut Emulator) -> Result<(), ()> {
        let (first, second, third, fourth) = self.decode(code);
        let (9, _, _, 0) = (first, second, third, fourth) else {
            return Err(());
        };

        let x = emulator.ram.get(second as usize);
        let y = emulator.ram.get(third as usize);

        if x != y {
            emulator.ram.set_pc(emulator.ram.get_pc() + 2);
        }

        Ok(())
    }
}
