use std::str;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use std::hash::{Hash, BuildHasherDefault};
use fnv::FnvHasher;


pub fn slice2str(data: &[u8]) -> Option<&str> {
    str::from_utf8(data).ok()
}

pub fn slice2string(data: &[u8]) -> Option<String> {
    String::from_utf8(data.to_vec()).ok()
}

pub fn handle_every_line(filepath: &str, func: &mut FnMut(String)) {
    if let Ok(f) = File::open(filepath) {
        let mut reader = BufReader::new(f);
        for line in reader.lines() {
            let mut line = match line {
                Ok(line) => line.trim().to_string(),
                _ => break,
            };

            func(line);
        }
    }
}


pub struct Dict<K, V> {
    map: HashMap<K, V, BuildHasherDefault<FnvHasher>>
}

impl<K, V> Dict<K, V>
    where K: Hash + Eq
{
    pub fn new() -> Self {
        Dict {
            map: HashMap::default()
        }
    }

    pub fn put(&mut self, k: K, v: V) {
        self.map.insert(k, v);
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        self.map.get(k)
    }

    pub fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        self.map.get_mut(k)
    }

    pub fn has(&self, k: &K) -> bool {
        if let Some(_) = self.get(k) {
            true
        } else {
            false
        }
    }
}