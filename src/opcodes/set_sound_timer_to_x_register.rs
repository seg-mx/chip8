use crate::{emulator::Emulator, opcode::Opcode};

pub struct SetSoundTimerToXRegister;

impl Opcode for SetSoundTimerToXRegister {
    fn execute(&self, code: u16, emulator: &mut Emulator) -> Result<(), ()> {
        let (first, second, third, fourth) = self.decode(code);
        let (0xF, _, 1, 8) = (first, second, third, fourth) else {
            return Err(());
        };

        let x = emulator.ram.get(second as usize);
        emulator.sound_timer = x;

        Ok(())
    }
}
