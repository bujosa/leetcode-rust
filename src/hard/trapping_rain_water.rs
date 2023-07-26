#![allow(dead_code)]

pub fn trap(height: Vec<i32>) -> i32 {
    todo!("Implement trap function")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trap() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(trap(height), 6);
    }
}
