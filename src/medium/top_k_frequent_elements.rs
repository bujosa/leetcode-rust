#![allow(dead_code)]
use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for num in nums {
        *map.entry(num).or_insert(0) += 1;
    }

    let mut pairs: Vec<(i32, i32)> = map.into_iter().collect();

    pairs.sort_by(|a, b| b.1.cmp(&a.1));
    pairs.into_iter().take(k as usize).map(|(k, _)| k).collect()
}

/*
   Algorithm 1: HashMap + Heap
   - Count the frequency of each number using a HashMap
   - Push the pairs of (number, frequency) into a max heap
   - Pop the top k elements from the heap and return the numbers

   Time: O(nlogn)
   Space: O(n)
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_frequent() {
        assert_eq!(top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
        assert_eq!(top_k_frequent(vec![1], 1), vec![1]);
    }
}
