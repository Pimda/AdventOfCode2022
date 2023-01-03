use std::collections::{HashMap, VecDeque};

pub enum DirectionalCollection<T> {
    Bfs(VecDeque<T>),
    Dfs(Vec<T>),
}

impl<T> DirectionalCollection<T> {
    pub fn dfs() -> Self {
        DirectionalCollection::Dfs(vec![])
    }

    pub fn bfs() -> Self {
        DirectionalCollection::Bfs(VecDeque::new())
    }

    pub fn push(&mut self, item: T) {
        match self {
            DirectionalCollection::Bfs(collection) => collection.push_back(item),
            DirectionalCollection::Dfs(collection) => collection.push(item),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self {
            DirectionalCollection::Bfs(collection) => collection.pop_front(),
            DirectionalCollection::Dfs(collection) => collection.pop(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            DirectionalCollection::Bfs(collection) => collection.len(),
            DirectionalCollection::Dfs(collection) => collection.len(),
        }
    }

    pub fn is_empty(&self) -> bool{
        match self {
            DirectionalCollection::Bfs(collection) => collection.is_empty(),
            DirectionalCollection::Dfs(collection) => collection.is_empty(),
        }
    }
}

pub struct PriorityQueue<T> {
    bins: HashMap<i32, VecDeque<T>>,
}

impl<T> PriorityQueue<T> {
    pub fn new() -> Self {
        Self {
            bins: HashMap::new(),
        }
    }

    pub fn push(&mut self, item: T, score: i32) {
        self.bins.entry(score).or_default().push_back(item);
    }

    pub fn pop_lowest(&mut self) -> Option<T> {
        let Some(lowest_key) = self.bins.keys().min() else{return None};
        self.pop(*lowest_key)
    }

    pub fn pop_highest(&mut self) -> Option<T> {
        let Some(highest_key) = self.bins.keys().max() else{return None};
        self.pop(*highest_key)
    }

    fn pop(&mut self, key: i32) -> Option<T> {
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

impl<T> Default for PriorityQueue<T>{
    fn default() -> Self {
        Self::new()
    }
}