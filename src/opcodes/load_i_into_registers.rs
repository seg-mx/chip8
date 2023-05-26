use crate::{emulator::Emulator, opcode::Opcode};

pub struct LoadIIntoRegisters;

impl Opcode for LoadIIntoRegisters {
    fn execute(&self, code: u16, emulator: &mut Emulator) -> Result<(), ()> {
        let (first, second, third, fourth) = self.decode(code);
        let (0xF, _, 6, 5) = (first, second, third, fourth) else {
            return Err(());
        };

        let x = second as usize;
        let i = emulator.ram.get_i_register() as usize;

        for index in 0..=x {
            emulator.ram.set(index, emulator.ram.get(index + i));
        }

        Ok(())
    }
}
