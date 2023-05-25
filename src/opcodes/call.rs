use crate::{emulator::Emulator, opcode::Opcode};

pub struct Call;

impl Opcode for Call {
    fn execute(&self, code: u16, emulator: &mut Emulator) -> Result<(), ()> {
        let (2, _, _, _) = self.decode(code) else {
            return Err(());
        };

        emulator.stack.push(emulator.ram.get_pc());
        emulator.ram.set_pc(code & 0x0FFF);

        Ok(())
    }
}
