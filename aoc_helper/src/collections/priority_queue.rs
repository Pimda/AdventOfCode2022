use std::collections::{HashMap, VecDeque};
use core::hash::Hash;

/// Divides items in bins, allowing to either retrieve an item for the lowest or highest key
/// Uses a VecDeque to save and return the elements, so per bin items are returned FiFo
pub struct PriorityQueue<K, T> {
    bins: HashMap<K, VecDeque<T>>,
}

impl<K, T> PriorityQueue<K, T> {
    pub fn new() -> Self {
        Self {
            bins: HashMap::new(),
        }
    }

    pub fn push(&mut self, item: T, score: K) where K: Eq + Hash {
        self.bins.entry(score).or_default().push_back(item);
    }

    pub fn pop_lowest(&mut self) -> Option<T> where K: Ord + Hash + Copy{
        let Some(lowest_key) = self.bins.keys().min() else{return None};
        self.pop(*lowest_key)
    }

    pub fn pop_highest(&mut self) -> Option<T> where K: Ord + Hash + Copy{
        let Some(highest_key) = self.bins.keys().max() else{return None};
        self.pop(*highest_key)
    }

    fn pop(&mut self, key: K) -> Option<T> where K: Eq + Hash{
        let maybe_bin = self.bins.get_mut(&key);
        match maybe_bin {
            Some(bin) => {
                let item = bin.pop_front();
                if bin.is_empty() {
                    self.bins.remove(&key);
                }
                item
            }
            None => panic!("Bin is empty"),
        }
    }
}

impl<K, T> Default for PriorityQueue<K, T>{
    fn default() -> Self {
        Self::new()
    }
}