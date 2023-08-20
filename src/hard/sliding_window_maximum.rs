#![allow(dead_code)]
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut result = Vec::new();
    let mut window = Vec::new();

    for i in 0..nums.len() {
        // Remove the first element if it is out of the window
        if i >= k as usize && window[0] <= i as i32 - k {
            window.remove(0);
        }

        // Remove all elements smaller than the current one
        while window.len() > 0 && nums[*window.last().unwrap() as usize] <= nums[i] {
            window.pop();
        }

        // Add the current element
        window.push(i as i32);

        // Add the maximum to the result
        if i >= k as usize - 1 {
            result.push(nums[*window.first().unwrap() as usize]);
        }
    }

    result
}

/*
    Algorithm - Sliding Window

    Time    O(N)
    Space   O(N)

    Description:
    1. We use a window to store the index of the elements.
    2. We remove the first element if it is out of the window.
    3. We remove all elements smaller than the current one.
    4. We add the current element.
    5. We add the maximum to the result.
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sliding_window() {
        assert_eq!(
            max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );

        assert_eq!(max_sliding_window(vec![1], 1), vec![1]);

        assert_eq!(max_sliding_window(vec![1, -1], 1), vec![1, -1]);

        assert_eq!(max_sliding_window(vec![9, 11], 2), vec![11]);
    }
}
