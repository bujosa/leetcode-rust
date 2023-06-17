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

/*
   Algorithm - O(1) time for all operations
   1. Use two stacks, one for the actual stack and one for the min stack
   2. When pushing, push to the actual stack and if the min stack is empty or the value is less than or equal to the top of the min stack, push to the min stack
   3. When popping, pop from the actual stack and if the value is equal to the top of the min stack, pop from the min stack
   4. When getting the top, return the top of the actual stack
   5. When getting the min, return the top of the min stack

   Time: O(1) for all operations
   Space: O(n) where n is the number of elements in the stack
*/

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
