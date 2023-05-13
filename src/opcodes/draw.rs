use crate::{
    emulator::Emulator,
    opcode::Opcode,
    screen::{SCREEN_HEIGHT, SCREEN_WIDTH},
};

pub struct Draw;

impl Opcode for Draw {
    fn execute(&self, code: u16, emulator: &mut Emulator) -> Result<(), ()> {
        let (first, second, third, rows) = self.decode(code);
        let (0xD, _, _, _) = (first, second, third, rows) else {
            return Err(());
        };

        emulator.ram.set(0xF, 0);

        let second = emulator.ram.get(second as usize) as u16;
        let third = emulator.ram.get(third as usize) as u16;
        let rows = rows as u16;

        for y in 0..rows {
            let addr = emulator.ram.get_i_register() + y as u16;
            let pixels = emulator.ram.get(addr as usize);

            for x in 0..8 {
                if (pixels & (0b1000_0000 >> x)) != 0 {
                    let x = (second + x) as usize % SCREEN_WIDTH;
                    let y = (third + y) as usize % SCREEN_HEIGHT;

                    let pixel = emulator.screen.get_pixel(x, y);

                    if pixel {
                        emulator.ram.set(0xF, 1);
                    }

                    emulator.screen.set_pixel(x, y, pixel ^ true);
                }
            }
        }

        Ok(())
    }
}
