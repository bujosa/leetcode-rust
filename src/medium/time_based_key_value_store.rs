#![allow(dead_code)]
struct TimeMap {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        TimeMap {}
    }

    fn set(&self, key: String, value: String, timestamp: i32) {}

    fn get(&self, key: String, timestamp: i32) -> String {
        "".to_string()
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_981() {
        let obj = TimeMap::new();
        obj.set("foo".to_string(), "bar".to_string(), 1);
        assert_eq!(obj.get("foo".to_string(), 1), "bar".to_string());
        assert_eq!(obj.get("foo".to_string(), 3), "bar".to_string());
        obj.set("foo".to_string(), "bar2".to_string(), 4);
        assert_eq!(obj.get("foo".to_string(), 4), "bar2".to_string());
        assert_eq!(obj.get("foo".to_string(), 5), "bar2".to_string());
    }
}
