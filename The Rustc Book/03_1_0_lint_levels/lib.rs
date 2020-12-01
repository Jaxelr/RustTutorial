//For all this scenarios you need to run from the console: rustc lib.rs --crate-type=lib

#![allow(missing_docs)]
#![allow(arithmetic_overflow)]
#![allow(unused_must_use)]
#![allow(unused_variables)]

// Allow linter example, change: #![allow(missing_docs)] to #![warn(missing_docs)] or #![deny(missing_docs)]
pub fn foo() {}

// Warn linter example, comment: #![allow(unused_variables)] to see
pub fn foo2() {
    let x = 5;
}

// Deny linter example comment: #![allow(arithmetic_overflow)] to see
pub fn foo3() {
    100u8 << 10;
}

