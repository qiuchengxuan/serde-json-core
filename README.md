Forked from [serde-json-core](serde-json-core),
removed heapless dependency and changed to using core::fmt::Formatter

[serde-json-core]: https://github.com/rust-embedded-community/serde-json-core/network

Usage
=====

```rust
struct AnyStruct {}

impl core::fmt::Display for AnyStruct {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        serde_json_core_fmt::to_fmt(f, self)
    }
}
```
