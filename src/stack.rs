pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    pub fn push(&mut self, new: T) {
        self.stack.push(new);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
    
    pub fn replace(&mut self, new: T) -> Option<T> {
        let out = self.stack.pop();
        self.stack.push(new);
        out
    }

    pub fn peek(&self) -> Option<&T> {
        self.stack.last()
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.stack.last_mut()
    }

    pub fn empty(&self) -> bool {
        self.stack.len() == 0
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }
}
