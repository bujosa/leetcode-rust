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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_11() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}
