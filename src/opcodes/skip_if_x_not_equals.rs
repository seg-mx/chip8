use crate::{emulator::Emulator, opcode::Opcode};

pub struct SkipIfXNotEquals;

impl Opcode for SkipIfXNotEquals {
    fn execute(&self, code: u16, emulator: &mut Emulator) -> Result<(), ()> {
        let (first, second, third, fourth) = self.decode(code);
        let (4, _, _, _) = (first, second, third, fourth) else {
            return Err(());
        };

        let x = emulator.ram.get(second as usize) as u16;

        if x != code & 0x00FF {
            emulator.ram.set_pc(emulator.ram.get_pc() + 2);
        }

        Ok(())
    }
}
