use std::{collections::HashMap, hash::Hash};

use cache::SizeLimitedCache;

use crate::ValueNode;

pub struct SeiveCache<Key, Value> 
where 
    Key: Eq + Hash,
    Value: Clone
{
    cache: HashMap::<Key, ValueNode<Value>>,
    seive_list: Vec::<Option<Key>>,
    hand: usize
}


impl<Key, Value> SeiveCache<Key, Value> 
where
    Key: Eq + Hash + Clone,
    Value: Clone
{
    pub fn new(capacity: usize) -> Self {
        SeiveCache { 
            cache: HashMap::new(), 
            seive_list: vec![None; capacity], 
            hand: 0,
        }        
    }

    fn add_key_to_seive(&mut self, key: Key) {
        let capacity = self.seive_list.capacity();
        
        while let Some(key) = &self.seive_list[self.hand] {
            if !self.cache.get(key).unwrap().take_read_state() {
                self.cache.remove(key);
                break;
            }

            self.hand = (self.hand + 1) % capacity;
        }

        self.seive_list[self.hand] = Some(key);
    }
}

impl<Key, Value> SizeLimitedCache<Key, Value>  for  SeiveCache<Key, Value> 
where
    Key: Eq + Hash + Clone,
    Value: Clone
{
    fn get(&self, key: &Key) -> Option<&Value> { 
        if let Some(v) = self.cache.get(key) {
            v.set_read_state();
            return Some(v.value())
        }

        None
    }


    fn set(&mut self, key: Key, value: Value) {
        // If hand is empty, insert; else, find the next candidate to remove
        let value_node = ValueNode::new(value);
        self.add_key_to_seive(key.clone());
        self.cache.insert(key, value_node);
    }

    fn cache(&self) -> Vec<(Key, Value)> {
        let mut result = Vec::<(Key, Value)>::with_capacity(self.cache.len());

        for (k, v) in &self.cache {
            result.push((k.clone(), v.value().clone()));
        }

        result
    }
    
}
