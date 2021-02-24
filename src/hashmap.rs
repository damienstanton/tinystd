// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[derive(Debug, Default, Hash, Clone)]
struct Bin<K, V>
where
    (K, V): Copy,
{
    data: Vec<(K, V)>,
}

impl<K, V> Bin<K, V>
where
    K: Eq,
    (K, V): Copy,
{
    fn get(&self, key: K) -> Option<&V> {
        for pair in &self.data {
            if pair.0 == key {
                return Some(&pair.1);
            }
        }
        None
    }
    fn set(&mut self, key: K, value: V) {
        for mut pair in &mut self.data {
            if pair.0 == key {
                pair.1 = value;
                return;
            }
        }
        self.data.push((key, value));
    }
    fn delete(&mut self, key: K) -> bool {
        let prelen = self.data.len();
        self.data = self
            .data
            .iter()
            .filter_map(|p| if p.0 != key { Some(*p) } else { None })
            .collect();
        let postlen = self.data.len();
        if prelen == postlen {
            false
        } else {
            true
        }
    }
}

#[derive(Debug)]
pub struct HashMap<K, V>
where
    K: Copy,
    V: Copy,
{
    hash_table: Vec<Bin<K, V>>,
    key_size: u64,
}

impl<K, V> HashMap<K, V>
where
    K: Clone + Copy + Default + Eq + Hash,
    V: Clone + Copy + Default + Eq,
{
    /// Constructs a new `HashMap<K,V>` using 92821 as the prime by which to
    /// modulo each key hash. This number was chosen based on the argument
    /// presented [in this Stack Overflow][1] discussion.
    ///
    /// [1]: https://stackoverflow.com/questions/1835976/what-is-a-sensible-prime-for-hashcode-calculation
    pub fn new() -> Self {
        let k = 92_821u64;
        let cells = vec![Bin::default(); k as usize];
        HashMap {
            hash_table: cells,
            key_size: k,
        }
    }

    /// Inserts `V` at key `K`.
    ///
    /// ## Example:
    /// ```
    /// use tinystd::hashmap::HashMap;
    ///
    /// let mut m = HashMap::<&str, i32>::new();
    /// m.insert("foo", 1);
    /// assert_eq!(m.get("foo"), Some(&1));
    /// ```
    pub fn insert(&mut self, key: K, value: V) {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let k = hasher.finish() % self.key_size;
        self.hash_table[k as usize].set(key, value);
    }

    /// Gets an optional reference to `V` using key `K`. If `K` doesn't exist
    /// in the map, `None` is returned.
    ///
    /// ## Example:
    /// ```
    /// use tinystd::hashmap::HashMap;
    ///
    /// let mut m = HashMap::<&str, i32>::new();
    /// m.insert("foo", 1);
    /// assert_eq!(m.get("foo"), Some(&1));
    /// assert_eq!(m.get("bar"), None);
    /// ```
    pub fn get(&mut self, key: K) -> Option<&V> {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let k = hasher.finish() % self.key_size;
        self.hash_table[k as usize].get(key)
    }

    /// Deletes the backing data store for the `(K, V)` pair in the map. Returns
    /// `true` if the key was found in the map --and thus deleted-- otherwise
    /// returns `false` if the key was not found in the map.
    /// Example:
    /// ```
    /// use tinystd::hashmap::HashMap;
    /// // Create and insert a test value
    /// let mut m = HashMap::<&str, i32>::new();
    /// m.insert("baz", 1);
    /// assert_eq!(m.get("baz"), Some(&1));
    ///
    /// // Check that "baz" really gets deleted, and that a double-delete is a no-op
    /// let deleted = m.remove("baz");
    /// assert_eq!(deleted, true);
    /// assert_eq!(m.get("baz"), None);
    /// assert_eq!(m.remove("baz"), false);
    /// ```
    pub fn remove(&mut self, key: K) -> bool {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let k = hasher.finish() % self.key_size;
        self.hash_table[k as usize].delete(key)
    }
}
