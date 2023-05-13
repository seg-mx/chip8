use std::io::{Result, Write};

use crate::{config::Config, quit, ram::Ram, screen::Screen};

pub struct Emulator {
    pub ram: Ram,
    pub screen: Screen,
    pub config: Config,
}

impl Emulator {
    pub fn new<W>(config: Config, output: &mut W) -> Self
    where
        W: Write,
    {
        let mut emulator = Self {
            ram: Ram::new(),
            screen: Screen::new(),
            config,
        };

        emulator
            .ram
            .load_rom(&emulator.config.rom)
            .unwrap_or_else(|err| quit(output, format_args!("{err}")));

        emulator
    }

    pub fn tick<W>(&mut self, w: &mut W) -> Result<()>
    where
        W: Write,
    {
        let code = self.ram.fetch();
        self.screen.redraw(w)
    }
}
