use std::{
    io::{Result, Write},
    time::Duration,
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

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
            if handle_key(key, emulator) {
                return Ok(true);
            }
        }
        Event::Resize(_, _) => emulator.screen.draw(output)?,
        _ => {}
    }

    Ok(false)
}

fn handle_key(key: KeyEvent, emulator: &mut Emulator) -> bool {
    if key.code == KeyCode::Char('c') && key.modifiers.intersects(KeyModifiers::CONTROL) {
        return true;
    }

    let pressed_key: Option<usize> = match key.code {
        KeyCode::Char('x') => Some(0),
        KeyCode::Char('1') => Some(1),
        KeyCode::Char('2') => Some(2),
        KeyCode::Char('3') => Some(3),
        KeyCode::Char('q') => Some(4),
        KeyCode::Char('w') => Some(5),
        KeyCode::Char('e') => Some(6),
        KeyCode::Char('a') => Some(7),
        KeyCode::Char('s') => Some(8),
        KeyCode::Char('d') => Some(9),
        KeyCode::Char('z') => Some(0xA),
        KeyCode::Char('c') => Some(0xB),
        KeyCode::Char('4') => Some(0xC),
        KeyCode::Char('r') => Some(0xD),
        KeyCode::Char('f') => Some(0xE),
        KeyCode::Char('v') => Some(0xF),
        _ => None,
    };

    if pressed_key.is_none() {
        return false;
    }

    let pressed_key = pressed_key.unwrap();

    match key.kind {
        KeyEventKind::Press => emulator.keypad.press(pressed_key),
        KeyEventKind::Release => emulator.keypad.release(pressed_key),
        _ => {}
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
