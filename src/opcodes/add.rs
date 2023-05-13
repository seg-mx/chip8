use crate::{emulator::Emulator, opcode::Opcode};

pub struct Add;

impl Opcode for Add {
    fn execute(&self, code: u16, emulator: &mut Emulator) -> Result<(), ()> {
        let (first, register, third, fourth) = self.decode(code);
        let (7, _, _, _) = (first, register, third, fourth) else {
            return Err(());
        };

        let register_value = emulator.ram.get(register as usize);
        let new_value = register_value.wrapping_add((code & 0x00FF) as u8);

        emulator.ram.set(register as usize, new_value);

        Ok(())
    }
}
