use crate::{emulator::Emulator, opcode::Opcode};

pub struct Jump;

impl Opcode for Jump {
    fn execute(&self, code: u16, emulator: &mut Emulator) -> Result<(), ()> {
        let (1, _, _, _) = self.decode(code) else {
            return Err(());
        };

        emulator.ram.set_pc(code & 0x0FFF);

        Ok(())
    }
}
