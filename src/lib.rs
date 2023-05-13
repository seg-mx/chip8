use std::{
    fmt::Arguments,
    io::{Result, Write},
    process,
};

use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

pub mod config;
pub mod emulator;
pub mod opcode;
pub mod ram;
pub mod screen;

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
    execute!(output, EnterAlternateScreen, Hide)?;
    enable_raw_mode()
}

pub fn restore<W>(output: &mut W) -> Result<()>
where
    W: Write,
{
    execute!(output, LeaveAlternateScreen, Show)?;
    disable_raw_mode()
}
