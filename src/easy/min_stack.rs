#![allow(dead_code)]
struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        if self.min_stack.is_empty() || val <= self.min_stack[self.min_stack.len() - 1] {
            self.min_stack.push(val);
        }
    }

    fn pop(&mut self) {
        let val = self.stack.pop().unwrap();
        if val == self.min_stack[self.min_stack.len() - 1] {
            self.min_stack.pop();
        }
    }

    fn top(&self) -> i32 {
        self.stack[self.stack.len() - 1]
    }

    fn get_min(&self) -> i32 {
        self.min_stack[self.min_stack.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_155() {
        let mut obj = MinStack::new();
        obj.push(-2);
        obj.push(0);
        obj.push(-3);
        assert_eq!(obj.get_min(), -3);
        obj.pop();
        assert_eq!(obj.top(), 0);
        assert_eq!(obj.get_min(), -2);
    }

    #[test]
    fn test_155_2() {
        let mut obj = MinStack::new();
        obj.push(0);
        obj.push(1);
        obj.push(0);
        assert_eq!(obj.get_min(), 0);
        obj.pop();
        assert_eq!(obj.get_min(), 0);
    }
}
