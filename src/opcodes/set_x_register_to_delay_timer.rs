use crate::{emulator::Emulator, opcode::Opcode};

pub struct SetXRegisterToDelayTimer;

impl Opcode for SetXRegisterToDelayTimer {
    fn execute(&self, code: u16, emulator: &mut Emulator) -> Result<(), ()> {
        let (first, second, third, fourth) = self.decode(code);
        let (0xF, _, 0, 7) = (first, second, third, fourth) else {
            return Err(());
        };

        emulator.ram.set(second as usize, emulator.delay_timer);

        Ok(())
    }
}
