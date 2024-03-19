# rust-gym

Fun little tasks to learn rust.

## Questions

Questions I'm exploring for further reading and exploration:
 - Why does the borrow checker check for borrows of immutable data?
    - Borrows of immutable data are fine. Moves of immutable data are fine. It's borrows after moves that the compiler doesn't allow. One owner, one compile-checked drop.
 - What are the apostrophes for `&str` in structs e.g. `&'static str`?
 - Where are different kinds of strings stored `String`, `&str`, `&'static str`? Heap, stack, ...?
    - `String` -> heap and ownership is checked
 - Why do some impls reference `&self`` and others `self`?

## Checklist

 - ✅ `arr.iter()` and `iter.map()`
 - ✅ number basics like `num.abs()`, `num_str.parse::<i32>().unwrap()`
 - ✅ tuple structs and C structs
 - ✅ `Result`, `unwrap()`, `is_ok()`, and `is_err()`
 - ✅ functions as first-class citizens, passing `fn` to `.map`
 - ✅ borrowing vecs `for el in &v` and consuming an iterator `v.into_iter()`

## Notes

p28 of `rust-by-example.pdf`