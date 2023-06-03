#[allow(dead_code)]
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;
    let mut i = digits.len() - 1;
    loop {
        if digits[i] == 9 {
            digits[i] = 0;
            if i == 0 {
                digits.insert(0, 1);
                break;
            }
            i -= 1;
        } else {
            digits[i] += 1;
            break;
        }
    }
    digits
}

/*
    Algorithm:
        - Initialize a variable i to the length of the digits vector minus 1
        - Loop
            - If the digit at index i is 9
                - Set the digit at index i to 0
                - If i is 0
                    - Insert 1 at index 0 of the digits vector
                    - Break out of the loop
                - Decrement i by 1
            - Else
                - Increment the digit at index i by 1
                - Break out of the loop
        - Return the digits vector

    Runtime: O(n)
    Space: O(1)
    best case: O(1)
    worst case: O(n)
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_one() {
        assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
        assert_eq!(plus_one(vec![0]), vec![1]);
        assert_eq!(plus_one(vec![9]), vec![1, 0]);
        assert_eq!(plus_one(vec![9, 9]), vec![1, 0, 0]);
        assert_eq!(plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
        assert_eq!(plus_one(vec![9, 9, 9, 9]), vec![1, 0, 0, 0, 0]);
        assert_eq!(plus_one(vec![9, 9, 9, 9, 9]), vec![1, 0, 0, 0, 0, 0]);
    }
}