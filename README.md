# assert-within

[![Crates.io](https://img.shields.io/crates/v/assert-within?style=flat-square)](https://crates.io/crates/assert-within)
[![Crates.io](https://img.shields.io/crates/d/assert-within?style=flat-square)](https://crates.io/crates/assert-within)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)

[API Docs](https://docs.rs/assert-within/latest/assert_within/)

A macro `assert_within!` for tests involving floating point numbers.

```rust
assert_within!(+0.001, val, target, "Value was not within additive 0.001: {more} {context}");
assert_within!(~0.05, val, target, "Value was not within 5% of target: {additional} {information:?}");
```

## Features

* Pass arguments by reference or value
* Sigils (`+`, `~`) indicate additive or relative error
* Traps NaN in any of the arguments
* Errors cause both the stringified expressions and their values to be displayed
* Arbitrary additional format args
* Generic over `num_traits::FloatCore`
* `no_std` compatible

## License

MIT or Apache 2.0 at your option.
