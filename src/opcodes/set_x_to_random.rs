use crate::{emulator::Emulator, opcode::Opcode};

pub struct SetXToRandom;

impl Opcode for SetXToRandom {
    fn execute(&self, code: u16, emulator: &mut Emulator) -> Result<(), ()> {
        let (first, second, third, fourth) = self.decode(code);
        let (0xC, _, _, _) = (first, second, third, fourth) else {
            return Err(());
        };

        let random: u8 = rand::random();
        let nn = (code & 0x00FF) as u8;
        let result = random & nn;

        emulator.ram.set(second as usize, result);

        Ok(())
    }
}
