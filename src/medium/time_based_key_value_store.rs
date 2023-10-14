#![allow(dead_code)]
use std::collections::HashMap;

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */
pub struct TimeMap {
    hm: HashMap<String, Vec<(String, i32)>>,
}

impl TimeMap {
    pub fn new() -> Self {
        Self { hm: HashMap::new() }
    }

    pub fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.hm.entry(key).or_default().push((value, timestamp));
    }

    pub fn get(&self, key: String, timestamp: i32) -> String {
        let mut res = String::new();

        if let Some(t_list) = self.hm.get(&key) {
            let (mut l, mut r) = (0, t_list.len());

            while l < r {
                let m = l + (r - l) / 2;
                if timestamp < t_list[m].1 {
                    r = m;
                } else {
                    res = t_list[m].0.clone();
                    l = m + 1;
                }
            }
        }

        res
    }
}

/*
    Algorithm - Binary Search

    - Create a HashMap with key as String and value as Vec<(String, i32)>
    - set(key, value, timestamp)
        - Insert the key into the HashMap if it doesn't exist
        - Push the value and timestamp into the Vec
    - get(key, timestamp)
        - If the key doesn't exist, return empty string
        - If the key exists, do binary search on the Vec<(String, i32)>
            - If the timestamp is less than the middle element's timestamp, search the left half
            - If the timestamp is greater than or equal to the middle element's timestamp, search the right half
            - If the timestamp is equal to the middle element's timestamp, return the value
            - If the timestamp is greater than the middle element's timestamp, return the value of the right element
            - If the timestamp is less than the middle element's timestamp, return the value of the left element

    Time    O(logN)
    Space   O(N)
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut obj = TimeMap::new();
        obj.set("foo".to_string(), "bar".to_string(), 1);
        assert_eq!(obj.get("foo".to_string(), 1), "bar".to_string());
        assert_eq!(obj.get("foo".to_string(), 3), "bar".to_string());
        obj.set("foo".to_string(), "bar2".to_string(), 4);
        assert_eq!(obj.get("foo".to_string(), 4), "bar2".to_string());
        assert_eq!(obj.get("foo".to_string(), 5), "bar2".to_string());
    }
}
