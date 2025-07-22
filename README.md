# KeyArray

[![crates.io](https://img.shields.io/crates/v/keyarray.svg)](https://crates.io/crates/keyarray) [![docs.rs](https://docs.rs/keyarray/badge.svg)](https://docs.rs/keyarray)

KeyArray is like a row of buttons—exactly one button (the **current key**) is “pressed” at any time. Use it to model mutually exclusive states, mode selectors, toggles, or feature flags in your Rust projects.

---

## Features

- Fixed-length list of keys where only one key is active  
- O(1) switching of the active key by index  
- Inspect current index, current key, or all keys  
- Dynamically push, insert, or remove keys at runtime  

---

## Installation

Add KeyArray to your project with Cargo:

```
[dependencies]
keyarray = "0.0.1-alpha"
```

---

## API Review

| Method                              | Example                                                      | Description                                              |
|-------------------------------------|--------------------------------------------------------------|----------------------------------------------------------|
| `KeyArray::new(keys: [K; N])`       | `let arr = KeyArray::new(["On", "Off", "Auto"]);`            | Create a new key array, defaulting to index 0            |
| `KeyArray::new_with(keys, i)`       | `let arr = KeyArray::new_with(["Low", "Med", "High"], 2);`   | Create with an explicit starting index                   |
| `change(i)`                         | `arr.change(1);`                                             | Switch the active key by index (panics if out of bounds) |
| `current_index()`                   | `let idx = arr.current_index();`                             | Get the currently active key’s index                     |
| `current()`                         | `let key = arr.current();`                                   | Get a reference to the currently active key              |
| `keys()`                            | `let all = arr.keys();`                                      | Borrow the slice of all keys                             |
| `push(key)`                         | `arr.push("Turbo");`                                         | Append a key to the end                                   |
| `insert(i, key)`                    | `arr.insert(1, "Inserted");`                                 | Insert a key at a specific index                         |
| `remove(i)`                         | `let removed = arr.remove(0);`                               | Remove and return the key at the given index             |

---

## Example

```
use keyarray::KeyArray;

fn main() {
    // Create with a default active key (index 0)
    let mut mykeys = KeyArray::new(["On", "Off", "Auto"]);

    // Create with an explicit starting index
    let mut other = KeyArray::new_with(["Low", "Medium", "High"], 2);

    // Change the active key by index (panics if out of bounds)
    mykeys.change(1);

    // Inspect the current state
    let idx = mykeys.current_index(); // 1
    let key = mykeys.current();        // "Off"
    let all = mykeys.keys();           // &["On", "Off", "Auto"]

    // Dynamically edit the key list
    mykeys.push("Turbo");              // ["On", "Off", "Auto", "Turbo"]
    mykeys.insert(1, "Inserted");      // ["On", "Inserted", "Off", ...]
    let removed = mykeys.remove(0);    // removes and returns "On"
}
```
