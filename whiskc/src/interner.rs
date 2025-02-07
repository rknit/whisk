use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

#[derive(Debug, Default, Clone)]
pub struct StringInterner {
    map: HashMap<String, u64>,
    hasher: DefaultHasher,
}
impl StringInterner {
    pub fn intern(&mut self, s: &str) -> u64 {
        if let Some(h) = self.map.get(s) {
            *h
        } else {
            s.hash(&mut self.hasher);
            let hash_val = self.hasher.finish();
            self.map.insert(s.to_owned(), hash_val);
            hash_val
        }
    }

    pub fn get(&self, s: &str) -> Option<u64> {
        self.map.get(s).copied()
    }
}
