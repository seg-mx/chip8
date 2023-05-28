use crate::{emulator::Emulator, opcode::Opcode};

pub struct JumpToZeroPlusNumber;

impl Opcode for JumpToZeroPlusNumber {
    fn execute(&self, code: u16, emulator: &mut Emulator) -> Result<(), ()> {
        let (0xB, _, _, _) = self.decode(code) else {
            return Err(());
        };

        let register = emulator.ram.get(0) as u16;
        let number = code & 0x0FFF;

        emulator.ram.set_pc(register + number);

        Ok(())
    }
}
