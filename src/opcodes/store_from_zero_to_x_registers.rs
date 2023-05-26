use crate::{emulator::Emulator, opcode::Opcode};

pub struct StoreFromZeroToXRegisters;

impl Opcode for StoreFromZeroToXRegisters {
    fn execute(&self, code: u16, emulator: &mut Emulator) -> Result<(), ()> {
        let (first, second, third, fourth) = self.decode(code);
        let (0xF, _, 5, 5) = (first, second, third, fourth) else {
            return Err(());
        };

        let i = emulator.ram.get_i_register();

        for index in 0..=(second as u16) {
            let register = emulator.ram.get(index as usize);

            emulator.ram.set((i + index) as usize, register);
        }

        Ok(())
    }
}
