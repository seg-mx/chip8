const STACK_SIZE: usize = 16;

pub struct Stack {
    stack: Vec<u16>,
}

impl Stack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn pop(&mut self) -> u16 {
        self.stack
            .pop()
            .expect("Stack pop should only be called with a non-empty stack")
    }

    pub fn push(&mut self, value: u16) {
        if self.stack.len() > STACK_SIZE {
            panic!(
                "Stack overflow has occured, last item is {}",
                self.stack.pop().unwrap()
            );
        }

        self.stack.push(value);
    }
}
