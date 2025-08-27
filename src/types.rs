use moka::future::Cache;
use std::sync::Arc;


// Cache key and value types
type Key = Arc<str>;
type Value = String;
pub type AppCache = Cache<Key, Value>;


/// Create a cache key from a string-like value
pub fn make_key<S>(s: S) -> Key
    where S: Into<Value>
{
    Arc::<str>::from(s.into())
}
