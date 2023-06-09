use std::io::{Result, Write};

use crate::{
    config::Config, keypad::Keypad, opcode::OpcodeManager, quit, ram::Ram, screen::Screen,
    stack::Stack,
};

pub struct Emulator {
    pub ram: Ram,
    pub screen: Screen,
    pub opcode_manager: Option<OpcodeManager>,
    pub keypad: Keypad,
    pub stack: Stack,
    pub delay_timer: u8,
    pub sound_timer: u8,
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
            opcode_manager: Some(OpcodeManager::new()),
            keypad: Keypad::new(),
            stack: Stack::new(),
            delay_timer: 0,
            sound_timer: 0,
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

        let mut manager = self
            .opcode_manager
            .take()
            .expect("OpcodeManager should never be a none variant in this context");

        manager.execute(code, self);

        self.opcode_manager = Some(manager);
        self.screen.redraw(w)
    }

    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }
}
