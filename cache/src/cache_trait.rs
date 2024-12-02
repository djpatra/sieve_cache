
pub trait SizeLimitedCache<Key, Value>
where
    Key: Eq + std::hash::Hash,
{
    fn get(&self, key: &Key) -> Option<&Value>;

    fn set(&mut self, key: Key, value: Value);

    fn cache(&self) -> Vec<(Key, Value)>;
}
