use crate::{emulator::Emulator, opcode::Opcode};

pub struct ReturnSubroutine;

impl Opcode for ReturnSubroutine {
    fn execute(&self, code: u16, emulator: &mut Emulator) -> Result<(), ()> {
        let (0, 0, 0xE, 0xE) = self.decode(code) else {
            return Err(());
        };

        emulator.ram.set_pc(emulator.stack.pop());

        Ok(())
    }
}
