#![allow(dead_code)]

pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    // Find the maximum number of bananas in a pile.
    let max = piles.iter().max().unwrap(); // O(n)

    // Binary search for the minimum eating speed.
    // The minimum eating speed is 1.
    let mut left = 1;

    // The maximum eating speed is the maximum number of bananas in a pile.
    let mut right = *max;

    while left < right {
        // The middle eating speed.
        let mid = left + (right - left) / 2;

        // The number of hours it takes to eat all bananas at the middle eating speed.
        let mut hours = 0;

        for pile in &piles {
            // If the number of bananas in the pile is divisible by the middle eating speed,
            // then the number of hours it takes to eat all bananas in the pile is the number
            // of bananas in the pile divided by the middle eating speed.
            if pile % mid == 0 {
                hours += pile / mid;
            } else {
                // Otherwise, the number of hours it takes to eat all bananas in the pile is
                // the number of bananas in the pile divided by the middle eating speed plus
                // one.
                hours += pile / mid + 1;
            }
        }

        // If the number of hours it takes to eat all bananas at the middle eating speed is
        // greater than the number of hours we have, then we need to increase the eating
        // speed.
        if hours > h {
            left = mid + 1;
        } else {
            // Otherwise, we need to decrease the eating speed.
            right = mid;
        }
    }

    left
}

/*
    Algorithm - Binary Search
    -------------------------

    Time Complexity: O(n log n)
    Space Complexity: O(1)
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_eating_speed() {
        assert_eq!(min_eating_speed(vec![3, 6, 7, 11], 8), 4);
        assert_eq!(min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
        assert_eq!(min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
    }
}
