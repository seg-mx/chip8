use crate::{emulator::Emulator, opcode::Opcode};

pub struct SetVRegister;

impl Opcode for SetVRegister {
    fn execute(&self, code: u16, emulator: &mut Emulator) -> Result<(), ()> {
        let (first, second, third, fourth) = self.decode(code);
        let (6, _, _, _) = (first, second, third, fourth) else {
            return Err(());
        };

        emulator.ram.set(second as usize, (code & 0x00FF) as u8);

        Ok(())
    }
}
