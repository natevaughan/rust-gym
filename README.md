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

## To practice
 - count characters in a string and return an arr of counts of letters of either case
    - can you return a borrowed array? Owned array? Only a heap vec? Boxed array?
 - find and extract all the ints in an array. Return an array with only those numbers.
 - iterate over a vec twice, once to print it and once to print it in reverse
 [x] fill a vec with n characters, where n is the letter of the alphabet sequence
 - Indexing into array must be `usize`, not `i32`
 - Various ways of looping over arrays and vecs (.map, iter(), borrow)
 - Mutating arrays in place
 - Creating an array filled with a specific value
 - .map_ok and .map_err for result

## Done

 - ✅ `arr.iter()` and `iter.map()`
 - ✅ number basics like `num.abs()`, `num_str.parse::<i32>().unwrap()`
 - ✅ tuple structs and C structs
 - ✅ `Result`, `unwrap()`, `is_ok()`, and `is_err()`
 - ✅ functions as first-class citizens, passing `fn` to `.map`
 - ✅ borrowing vecs `for el in &v` and consuming an iterator `v.into_iter()`

## Notes

p28 of `rust-by-example.pdf`