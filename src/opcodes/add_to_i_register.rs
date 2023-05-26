use crate::{emulator::Emulator, opcode::Opcode};

pub struct AddToIRegister;

impl Opcode for AddToIRegister {
    fn execute(&self, code: u16, emulator: &mut Emulator) -> Result<(), ()> {
        let (first, second, third, fourth) = self.decode(code);
        let (0xF, _, 1, 0xE) = (first, second, third, fourth) else {
            return Err(());
        };

        let i = emulator.ram.get_i_register();
        let result = i.wrapping_add(second as u16);
        emulator.ram.set_i_register(result);

        Ok(())
    }
}
