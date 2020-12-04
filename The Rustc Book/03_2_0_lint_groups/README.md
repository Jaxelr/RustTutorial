# Lint Groups

The following lint groups can be toggled as arguments for rustc:

| Command | Description |
| :---: | :---: |
| `rustc -D warnings main.rs` | All lints warnings |
| `rustc -D future-incompatible main.rs` | All lints that have future incompatibles |
| `rustc -D nonstandard-style main.rs` | All lints that have non standard naming |
| `rustc -D rust-2018-compatibility main.rs` | All lints that were used to transition from the 2015 edition to the 2018 edition |
| `rustc -D rust-2018-idioms main.rs` | All lints that nudge you to idiomatic features of Rust 2018 edition |
| `rustc -D rustdoc main.rs` | Rustdoc specific lints |
| `rustc -D unused main.rs` | Lints that detect excess syntax |
