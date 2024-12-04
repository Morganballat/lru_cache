use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{self, Write};

use crate::cache_trait::CacheTrait;

pub struct LRUCache<V: std::fmt::Debug> {
    capacity: usize,
    map: HashMap<String, V>,
    order: Vec<String>,
    pub filename: Option<String>,
}

impl<V: std::fmt::Debug + std::str::FromStr> CacheTrait<V> for LRUCache<V> {
    fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            map: HashMap::new(),
            order: Vec::new(),
            filename: None,
        }
    }

    fn get(&mut self, key: &str) -> Option<&V> {
        if self.map.contains_key(key) {
            self.order.retain(|k| k != &key);
            self.order.push(key.to_string());
            self.map.get(key)
        } else {
            None
        }
    }

    fn put(&mut self, key: &str, value: V) {
        if self.map.contains_key(key) {
            self.order.retain(|k| k != &key);
        } else if self.map.len() == self.capacity {
            let lru_key = self.order.remove(0);
            self.map.remove(&lru_key);
        }
        self.map.insert(key.to_string(), value);
        self.order.push(key.to_string());

        }
    
    fn save_to_file(&self) -> io::Result<()> {
        if let Some(filename) = &self.filename {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(filename)?;
            for (key, value) in &self.map {
                writeln!(file, "{}:{:?}", key, value)?;
            }
        }
        Ok(())
    }
        
}