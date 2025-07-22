/// KeyArray is like a row of buttons; exactly one button (the current key)
/// is “pressed” at any time.
///
/// To create (defaults to first key):
///     let mut mykeys = KeyArray::new(["On", "Off", "Auto"]);
///
/// To create with an explicit start:
///     let mut mykeys = KeyArray::new_with(["On", "Off"], 1);
///
/// To change the status (current key by index):
///     mykeys.change(2);
///
/// To inspect:
///     let idx = mykeys.current_index();
///     let key = mykeys.current();
///     let all = mykeys.keys();
///
/// To edit the key list:
///     mykeys.push("New");
///     mykeys.insert(1, "Inserted");
///     let removed = mykeys.remove(0);
///

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

pub struct KeyArray<K> {
    keys: Vec<K>,
    idx: usize,
}

impl<K> KeyArray<K>
where
    K: Clone + PartialEq + Debug + Display,
{
    /// Create from any iterable of keys. Panics if empty.
    pub fn new(keys: impl IntoIterator<Item = K>) -> Self {
        let keys: Vec<K> = keys.into_iter().collect();
        assert!(!keys.is_empty(), "KeyArray::new: must supply at least one key");
        KeyArray { keys, idx: 0 }
    }

    /// Same as `new`, but start at `start_idx`. Panics if out of bounds.
    pub fn new_with(keys: impl IntoIterator<Item = K>, start_idx: usize) -> Self {
        let keys: Vec<K> = keys.into_iter().collect();
        assert!(!keys.is_empty(), "KeyArray::new_with: must supply keys");
        assert!(
            start_idx < keys.len(),
            "KeyArray::new_with: start_idx {} out of bounds",
            start_idx
        );
        KeyArray { keys, idx: start_idx }
    }

    /// Change the current key by zero‐based index.
    /// Panics if `i` is out of bounds.
    pub fn change(&mut self, i: usize) {
        assert!(
            i < self.keys.len(),
            "KeyArray::change: index {} out of bounds",
            i
        );
        self.idx = i;
    }

    /// Get a reference to the current key.
    pub fn current(&self) -> &K {
        &self.keys[self.idx]
    }

    /// Get the index of the current key.
    pub fn current_index(&self) -> usize {
        self.idx
    }

    /// Get a slice of all keys.
    pub fn keys(&self) -> &[K] {
        &self.keys
    }

    /// Number of keys.
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    /// Append a new key after the last.
    pub fn push(&mut self, key: K) {
        self.keys.push(key);
    }

    /// Insert a key at position `i`. Panics if `i > len`.
    pub fn insert(&mut self, i: usize, key: K) {
        assert!(i <= self.keys.len(), "KeyArray::insert: index {} out of bounds", i);
        self.keys.insert(i, key);
        // if you inserted before current idx, bump it forward
        if i <= self.idx {
            self.idx += 1;
        }
    }

    /// Remove and return the key at `i`. Panics if out of bounds.
    pub fn remove(&mut self, i: usize) -> K {
        assert!(i < self.keys.len(), "KeyArray::remove: index {} out of bounds", i);
        let removed = self.keys.remove(i);
        // adjust current index
        if self.idx >= self.keys.len() {
            // if we removed the last element, clamp idx
            self.idx = self.keys.len() - 1;
        }
        removed
    }
}

impl<K> Display for KeyArray<K>
where
    K: Clone + PartialEq + Debug + Display,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "keys={:?}, current_idx={}, current={}",
            self.keys,
            self.idx,
            self.current()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_flow() {
        let mut ka = KeyArray::new(["A", "B", "C"]);
        assert_eq!(ka.current(), &"A");
        assert_eq!(ka.current_index(), 0);

        ka.change(2);
        assert_eq!(ka.current(), &"C");

        // push a new key
        ka.push("D");
        assert_eq!(ka.len(), 4);
        assert_eq!(ka.keys(), &["A", "B", "C", "D"]);

        // insert at front
        ka.insert(0, "X");
        assert_eq!(ka.keys()[0], "X");
        // current was 2→ now is 3
        assert_eq!(ka.current(), &"C");

        // remove the first
        let removed = ka.remove(0);
        assert_eq!(removed, "X");
        assert_eq!(ka.keys()[0], "A");
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn change_oob_panics() {
        let mut ka = KeyArray::new(["Only"]);
        ka.change(5);
    }

    #[test]
    fn display_format() {
        let ka = KeyArray::new(["Up", "Down"]);
        let s = format!("{}", ka);
        assert!(s.contains(r#"["Up", "Down"]"#) && s.contains("current_idx=0"));
    }
}
