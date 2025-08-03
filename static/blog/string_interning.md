---
title: String interning
author: Pascal Behmenburg
date: 2025-08-04
---
# String interning { #string-interning }

~written on August 04, 2025 by Pascal Behmenburg~

---

# A String interner { #a-string-interner }

A string interner is a data structure that stores a set of strings and provides a way to efficiently retrieve the canonical representation of a string. It is useful for reducing memory usage and improving performance when dealing with large numbers of strings.

<center>
    <img src="./img/string_interning.png" alt="String interning" width="50%" height="50%">
</center>

I once came across them, when I was interested in storing file paths for a code search index efficiently. I toyed around with different kinds of implementations, both thread-safe and not. For simplicity, I will show you a variant which is not thread-safe and dynamically allocates more memory as needed. It is unsafe though as it does leak references to the buffer where strings are stored. This is fine in some cases, e.g. when you don't need to share the interner between threads and you can guarantee that the interner will outlive all references to the interned strings.


```rs
use std::collections::HashMap;
use std::str;

pub struct StringInterner {
    buffer: Vec<u8>,
    map: HashMap<&'static str, &'static str>,
}

impl StringInterner {
    pub fn new(initial_capacity: usize) -> Self {
        let mut buffer = Vec::with_capacity(initial_capacity);
        unsafe { buffer.set_len(0) };

        Self {
            buffer,
            map: HashMap::new(),
        }
    }

    pub fn intern(&mut self, s: &str) -> &'static str {
        if let Some(&interned) = self.map.get(s) {
            return interned;
        }

        let start = self.buffer.len();
        self.buffer.extend_from_slice(s.as_bytes());
        let end = self.buffer.len();

        unsafe {
            let slice = &self.buffer[start..end];
            let interned = str::from_utf8_unchecked(slice);
            let static_ref = std::mem::transmute::<&str, &'static str>(interned);
            self.map.insert(static_ref, static_ref);
            static_ref
        }
    }
}
```

As you can see, the `intern` method takes a string slice `s` and returns a static reference to the interned string. If the string is already interned, it returns the existing interned string. Otherwise, it adds the string to the buffer and returns a static reference to the interned string.

This is by no means a solution to be used in practice, but there are many available on crates.io if you need a thread- and memory-safe solution.
