use crate::{emulator::Emulator, opcode::Opcode};

pub struct SetIRegister;

impl Opcode for SetIRegister {
    fn execute(&self, code: u16, emulator: &mut Emulator) -> Result<(), ()> {
        let (0xA, _, _, _) = self.decode(code) else {
            return Err(());
        };

        emulator.ram.set_i_register(code & 0x0FFF);

        Ok(())
    }
}
