use std::io::{self};

pub trait CacheTrait<V> {
    fn new(capacity: usize) -> Self;
    fn save_to_file(&self) -> io::Result<()>;
    fn get(&mut self, key: &str) -> Option<&V>;
    fn put(&mut self, key: &str, value: V);
}