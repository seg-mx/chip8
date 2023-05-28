use crate::{emulator::Emulator, opcode::Opcode};

pub struct SetIToBcdOfXRegister;

impl Opcode for SetIToBcdOfXRegister {
    fn execute(&self, code: u16, emulator: &mut Emulator) -> Result<(), ()> {
        let (first, second, third, fourth) = self.decode(code);
        let (0xF, _, 3, 3) = (first, second, third, fourth) else {
            return Err(());
        };

        let x = emulator.ram.get(second as usize) as f32;

        let hundreds = (x / 100.0).floor() as u8;
        let tens = ((x / 10.0) % 10.0).floor() as u8;
        let ones = (x % 10.0).floor() as u8;

        let i = emulator.ram.get_i_register() as usize;

        emulator.ram.set(i, hundreds);
        emulator.ram.set(i + 1, tens);
        emulator.ram.set(i + 2, ones);

        Ok(())
    }
}
