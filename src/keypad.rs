const NUMBER_OF_KEYS: usize = 16;

pub struct Keypad {
    keys: [bool; NUMBER_OF_KEYS],
}

impl Keypad {
    pub fn new() -> Self {
        Self {
            keys: [false; NUMBER_OF_KEYS],
        }
    }

    pub fn is_pressed(&self, key: usize) -> bool {
        self.keys[key]
    }

    pub fn press(&mut self, key: usize) {
        self.keys[key] = true;
    }

    pub fn release(&mut self, key: usize) {
        self.keys[key] = false;
    }
}
