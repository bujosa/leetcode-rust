#![allow(dead_code)]
pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_area = 0;
    let mut left = 0;
    let mut right = height.len() - 1;

    while left < right {
        let area = (right - left) as i32 * std::cmp::min(height[left], height[right]);
        max_area = std::cmp::max(max_area, area);

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max_area
}

/*
    Algorithm - Two Pointers

    Time    O(N)
    Space   O(1)

    1. left = 0, right = len-1
    2. loop while left < right
        3. area = (right-left) * min(height[left], height[right])
        4. maxArea = max(maxArea, area)
        5. if height[left] < height[right]
            6. left++
        7. else
            8. right--
    9. return maxArea
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_11() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}
