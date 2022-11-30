fn main() {
    let mut cache = LRUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    println!("{}", cache.get(1));
    cache.put(3, 3);
    println!("{}", cache.get(2));
    cache.put(4, 4);
    println!("{}", cache.get(1));
    println!("{}", cache.get(3));
    println!("{}", cache.get(4));

    let mut cache = LRUCache::new(2);
    cache.put(1, 0);
    cache.put(2, 2);
    println!("{}", cache.get(1));
    cache.put(3, 3);
    println!("{}", cache.get(2));
    cache.put(4, 4);
    println!("{}", cache.get(1));
    println!("{}", cache.get(3));
    println!("{}", cache.get(4));

    let mut cache = LRUCache::new(2);
    cache.put(2, 1);
    cache.put(2, 2);
    println!("{}", cache.get(2));
    cache.put(1, 1);
    cache.put(4, 1);
    println!("{}", cache.get(2));
}

use std::collections::HashMap;
struct LRUCache {
    used_capacity: usize,
    max_capacity: usize,
    keys_sorted_by_lru: Vec<i32>,
    map: HashMap<i32, i32>,
}
impl LRUCache {
    fn new(capacity: i32) -> Self {
        let max_capacity = capacity as usize;
        Self {
            used_capacity: 0,
            max_capacity,
            keys_sorted_by_lru: vec![],
            map: HashMap::with_capacity(max_capacity),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.map.get(&key) {
            Some(&val) => {
                let index = self
                    .keys_sorted_by_lru
                    .iter()
                    .position(|&element| element == key)
                    .unwrap();
                self.keys_sorted_by_lru.remove(index);
                self.keys_sorted_by_lru.push(key);
                val
            }
            None => -1,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        // if it's in the map, find it in the list and temporarily remove it from the lru key list
        match self.map.get(&key) {
            Some(&val) => {
                let index = self
                    .keys_sorted_by_lru
                    .iter()
                    .position(|&element| element == key)
                    .unwrap();
                self.keys_sorted_by_lru.remove(index);
                if val != value {
                    self.map.insert(key, value);
                }
            }
            None => {
                // if it's not in the map, put it in the map
                self.map.insert(key, value);
                if self.used_capacity < self.max_capacity {
                    // and if we haven't used our whole capacity, increment capacity
                    self.used_capacity += 1;
                } else {
                    // but if we have used our whole capacity, remove the lru key from the map and list
                    self.map.remove(&self.keys_sorted_by_lru[0]);
                    self.keys_sorted_by_lru.remove(0);
                }
            }
        }
        // regardless of whether we had to put it in the map, it now goes to the end of the lru key list
        self.keys_sorted_by_lru.push(key);
    }
}
