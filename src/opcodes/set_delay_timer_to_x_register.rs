use crate::{emulator::Emulator, opcode::Opcode};

pub struct SetDelayTimerToXRegister;

impl Opcode for SetDelayTimerToXRegister {
    fn execute(&self, code: u16, emulator: &mut Emulator) -> Result<(), ()> {
        let (first, second, third, fourth) = self.decode(code);
        let (0xF, _, 1, 5) = (first, second, third, fourth) else {
            return Err(());
        };

        let x = emulator.ram.get(second as usize);
        emulator.delay_timer = x;

        Ok(())
    }
}
