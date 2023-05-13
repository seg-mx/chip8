use crate::{emulator::Emulator, opcode::Opcode};

pub struct ClearScreen;

impl Opcode for ClearScreen {
    fn execute(&self, code: u16, emulator: &mut Emulator) -> Result<(), ()> {
        let (0, 0, 0xE, 0) = self.decode(code) else {
            return Err(());
        };

        emulator.screen.clear();

        Ok(())
    }
}
