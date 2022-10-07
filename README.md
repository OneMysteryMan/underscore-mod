# underscore-mod

A macro to change the path of `module/mod.rs` to `module/_mod.rs`

## Example

```
root
└─┬─ main.rs
  └─ foobar/
     └┬─ _mod.rs
      ├─ bar.rs
      └─ foo.rs
```

**main.rs:**

```rust
_mod!(foobar);
//...
use underscore_mod::_mod;
//...
fn main() {}
```
