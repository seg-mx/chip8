use crate::{emulator::Emulator, keypad::NUMBER_OF_KEYS, opcode::Opcode};

pub struct WaitForKey;

impl Opcode for WaitForKey {
    fn execute(&self, code: u16, emulator: &mut Emulator) -> Result<(), ()> {
        let (first, second, third, fourth) = self.decode(code);
        let (0xF, _, 0, 0xA) = (first, second, third, fourth) else {
            return Err(());
        };

        let x = second as usize;
        let mut any_key_is_pressed = false;

        for i in 0..NUMBER_OF_KEYS {
            if emulator.keypad.is_pressed(i) {
                emulator.ram.set(x, i as u8);
                any_key_is_pressed = true;
                break;
            }
        }

        if !any_key_is_pressed {
            emulator.ram.set_pc(emulator.ram.get_pc() - 2);
        }

        Ok(())
    }
}
