mod medium;

fn main() {
    let mut cache = medium::lru_cache::LRUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    println!("{}", cache.get(1)); // returns 1
    cache.put(3, 3); // evicts key 2
    println!("{}", cache.get(2)); // returns -1 (not found)
    cache.put(4, 4); // evicts key 1
    println!("{}", cache.get(1)); // returns -1 (not found)
    println!("{}", cache.get(3)); // returns 3
    println!("{}", cache.get(4)); // returns 4
}
