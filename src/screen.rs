use std::{
    collections::HashMap,
    io::{Result, Write},
};

use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Print, Stylize},
};

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

pub struct Screen {
    pixels: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT],
    to_change: HashMap<(usize, usize), bool>,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            pixels: [[false; SCREEN_WIDTH]; SCREEN_HEIGHT],
            to_change: HashMap::new(),
        }
    }

    fn draw_pixel<W>(w: &mut W, x: u16, y: u16, pixel: bool) -> Result<()>
    where
        W: Write,
    {
        queue!(
            w,
            MoveTo(x, y),
            Print(if pixel {
                " ".on_white()
            } else {
                " ".on_black()
            })
        )
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: bool) {
        self.pixels[y][x] = pixel;
        self.to_change.insert((x, y), pixel);
    }

    pub fn clear(&mut self) {
        self.pixels = [[false; SCREEN_WIDTH]; SCREEN_HEIGHT];
    }

    pub fn draw<W>(&self, w: &mut W) -> Result<()>
    where
        W: Write,
    {
        for (y, line) in self.pixels.iter().enumerate() {
            for (x, pixel) in line.iter().enumerate() {
                Self::draw_pixel(w, x as u16, y as u16, *pixel)?;
            }
        }

        w.flush()?;

        Ok(())
    }

    pub fn redraw<W>(&mut self, w: &mut W) -> Result<()>
    where
        W: Write,
    {
        if self.to_change.is_empty() {
            return Ok(());
        }

        for ((x, y), pixel) in self.to_change.iter() {
            Self::draw_pixel(w, *x as u16, *y as u16, *pixel)?;
        }

        w.flush()?;

        self.to_change.clear();

        Ok(())
    }
}