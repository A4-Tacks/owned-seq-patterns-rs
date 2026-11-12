Some helper functions, owned matching seq values

Examples
-------------------------------------------------------------------------------
Slice patterns can used on `&Vec<String>`:

```rust
let vec: &Vec<String> = &vec!["a".into(), "b".into(), "c".into(), "d".into(), "e".into()];
let [a, b, s @ .., e] = &vec[..] else { panic!() };

assert_eq!(a, "a");
assert_eq!(b, "b");
assert_eq!(e, "e");
assert_eq!(core::any::type_name_of_val(&a), "&alloc::string::String");
assert!(s.into_iter().eq(["c", "d"]));
```

But can't work on `Vec<String>`:

```rust,compile_fail
let vec: Vec<String> = vec!["a".into(), "b".into(), "c".into(), "d".into(), "e".into()];
let [a, b, .., e] = vec else { panic!() };
```

You can use this crate:

```rust
use owned_seq_patterns::OwnedSeqPatternHelper;

let vec: Vec<String> = vec!["a".into(), "b".into(), "c".into(), "d".into(), "e".into()];
let Ok(([a, b], s, [e])) = vec.heads_tails_with() else { panic!() };

assert_eq!(a, "a".to_owned());
assert_eq!(b, "b".to_owned());
assert_eq!(e, "e".to_owned());
assert_eq!(core::any::type_name_of_val(&a), "alloc::string::String");
assert!(s.into_iter().eq(["c".to_owned(), "d".to_owned()]));
```
