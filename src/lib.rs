use std::{
    fmt::Arguments,
    io::{Result, Write},
    process,
    time::{Duration, Instant},
};

use crossterm::{
    cursor::{Hide, Show},
    event::{KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use emulator::Emulator;
use event_handler::handle_event;

pub mod config;
pub mod emulator;
pub mod event_handler;
pub mod keypad;
pub mod opcode;
pub mod opcodes;
pub mod ram;
pub mod screen;
pub mod stack;

pub fn run<W>(mut emulator: Emulator, output: &mut W) -> Result<()>
where
    W: Write,
{
    let mut last_tick = Instant::now();

    loop {
        let timeout = emulator
            .config
            .tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::ZERO);

        if last_tick.elapsed() >= emulator.config.tick_rate {
            emulator.tick(output)?;
            last_tick = Instant::now();
        }

        if handle_event(timeout, &mut emulator, output)? {
            break;
        }
    }

    restore(output)?;

    Ok(())
}

pub fn quit<W>(output: &mut W, err: Arguments) -> !
where
    W: Write,
{
    if let Err(err) = restore(output) {
        eprintln!("Error while restoring terminal: {err}\n");
    }

    eprintln!("{err}");
    process::exit(1);
}

pub fn setup<W>(output: &mut W) -> Result<()>
where
    W: Write,
{
    execute!(
        output,
        EnterAlternateScreen,
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::all()),
        Hide,
    )?;
    enable_raw_mode()
}

pub fn restore<W>(output: &mut W) -> Result<()>
where
    W: Write,
{
    execute!(
        output,
        PopKeyboardEnhancementFlags,
        LeaveAlternateScreen,
        Show
    )?;
    disable_raw_mode()
}
