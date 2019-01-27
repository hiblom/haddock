use std::collections::HashMap;

#[derive(Clone)]
pub struct HashCounter(HashMap<u64, i32>);

impl HashCounter {
    pub fn new() -> HashCounter {
        HashCounter(HashMap::new())
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn incr(&mut self, hash: u64) {
        *self.0.entry(hash).or_insert(0) += 1;
    }

    pub fn decr(&mut self, hash: u64) {
        match self.0.get_mut(&hash) {
            Some(c) => {
                if *c == 1 {
                    self.0.remove(&hash);
                }
                else {
                    *c -= 1;
                }
            },
            None => {
                println!("hash {:X} not found while decr", hash);
            }
        }
    }

    pub fn get(&self, hash: u64) -> i32 {
        match self.0.get(&hash) {
            Some(c) => *c,
            None => 0
        }
    }

    pub fn get_len(&self) -> i32 {
        let mut size = 0;
        for (_, c) in self.0.iter() {
            size += c;
        }
        size
    }
}