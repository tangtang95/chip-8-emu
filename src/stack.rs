pub struct Stack<T> {
    vector: Vec<T>
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Self { vector: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.vector.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.vector.pop()
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::Stack;

    #[test]
    fn push_test() {
        let mut stack: Stack<u16> = Stack::new();
        stack.push(16);

        assert!(stack.vector[0] == 16);
    }

}