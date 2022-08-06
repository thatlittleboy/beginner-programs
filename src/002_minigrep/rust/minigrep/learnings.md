# Learning points from the tutorial
This document is a live record of the questions that I had when going through the tutorial.

---

In our main function, we first collect the input command line arguments into a Vector of String objects, then attempt to parse out the query arg and the filename arg.
```rust
let args: Vec<String> = env::args().collect();

let query: &String = &args[1];
let filename: &String = &args[2];
```
Q:
Why is there a need to use `&args[1]`, instead of `args[1]` directly here? What error does the latter result in?

A:
Without the `&`, the assignment results in a move of the `String` object, but cannot move out of index of `Vec<String>`.
And `String` does not implement the `Copy` trait, hence compiler throws an error.

---
