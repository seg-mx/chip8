use std::{
    io::{Result, Write},
    time::Duration,
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

use crate::emulator::Emulator;

pub fn handle_event<W>(timeout: Duration, emulator: &mut Emulator, output: &mut W) -> Result<bool>
where
    W: Write,
{
    let Some(event) = read_event(timeout) else {
        return Ok(false);
    };

    match event {
        Event::Key(key) => {
            if handle_key(key) {
                return Ok(true);
            }
        }
        Event::Resize(_, _) => emulator.screen.draw(output)?,
        _ => {}
    }

    Ok(false)
}

fn handle_key(key: KeyEvent) -> bool {
    if key.code == KeyCode::Char('c') && key.modifiers.intersects(KeyModifiers::CONTROL) {
        return true;
    }

    false
}

fn read_event(timeout: Duration) -> Option<Event> {
    match event::poll(timeout) {
        Ok(has_event) => {
            if !has_event {
                return None;
            }
        }
        Err(_) => return None,
    }

    let event = match event::read() {
        Ok(event) => event,
        Err(_) => return None,
    };

    Some(event)
}
