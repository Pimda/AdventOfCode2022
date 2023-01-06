use std::collections::VecDeque;

/// Helps to quickly switch between dfs and bfs, using simple Vec and VecDeque
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