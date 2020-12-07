#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_variables)]
#![allow(unused_imports)]
//#![feature(unsafe_block_in_unsafe_fn)] //Needs unstable channel

//#![deny(elided_lifetimes_in_paths)] //This points to #Example5 but must be at crate level
//#![deny(explicit_outlives_requirements)] //This points to #Example6 but must be at crate level
//#![deny(non_ascii_idents)] //This points to #Example11 but must be at crate level
//#![deny(trivial_casts)] //This points to #Example14 but must be at crate level
//#![deny(trivial_numeric_casts)] //This points to #Example15 but must be at crate level
//#![deny(unaligned_references)] //This points to #Example16 but must be at crate level
//#![deny(unsafe_code)] //This points to #Example18 but must be at crate level
//#![deny(unused_qualifications)] //This points to #Example21 but must be at crate level
//#![deny(unused_results)] //This points to #Example22 but must be at crate level
//#![deny(variant_size_differences)] //This points to #Example23 but must be at crate level

//#![deny(absolute_paths_not_starting_with_crate)]
mod foo {
    pub fn bar() {}
}

//#![deny(anonymous_parameters)]
pub trait Foo2 {
    fn foo(usize);
}

//#![deny(box_pointers)]
struct Foo3 {
    x: Box<isize>,
}

//#Example5
struct Foo4<'a> {
    x: &'a u32
}

fn foo5(x: &Foo4) {
}

//#Example6
struct SharedRef<'a, T>
where
    T: 'a,
{
    data: &'a T,
}

//#![deny(keyword_idents)]
fn dyn() {}

//#![deny(macro_use_extern_crate)]
//#[macro_use]
//extern crate serde_json;

//#![deny(meta_variable_misuse)]
macro_rules! foo6 {
    () => {};
    ($( $i:ident = $($j:ident),+ );*) => { $( $( $i = $k; )+ )* };
}

//#![deny(missing_copy_implementations)]
pub struct Foo7 {
    pub field: i32
}

//#![deny(missing_debug_implementations)]
pub struct Foo8;

//#![deny(missing_docs)]
pub fn foo9() {}

//#Example11
// #![feature(non_ascii_idents)]
// fn foo10() {
//     let föö = 1;
// }

//#![deny(pointer_structural_match)]
fn foo11(a: usize, b: usize) -> usize { a + b }
const FOO12: fn(usize, usize) -> usize = foo11;

//#![deny(single_use_lifetimes)]
fn foo13<'a>(x: &'a u32) {}

#[repr(packed)]
pub struct Foo14 {
    field1: u64,
    field2: u8,
}

//#![deny(unreachable_pub)]
mod foo15 {
    pub mod bar {

    }
}

unsafe fn foo16() {}

//#![deny(unsafe_op_in_unsafe_fn)]
unsafe fn foo17() {
    foo16();
}

//#![deny(unused_import_braces)]
pub mod foo18 {
    pub struct A;
}

use foo18::{A};

//#[deny(unused_lifetimes)]
pub fn foo19<'a>() {}

//#Example21
mod foo20 {
    pub fn bar() {}
}

//#Example22
fn foo21<T>() -> T { panic!() }

//#Example23
enum Foo22 {
    V0(u8),
    VBig([u8; 1024]),
}

fn main() {
    ::foo::bar();

    match FOO12 {
        FOO12 => {},
        _ => {},
    }

    //#Example14
    let x: &u32 = &42;
    let y = x as *const u32;

    //#Example15
    let x2 = 42_i32 as i32;

    //#Example18
    unsafe {
        let foo14 = Foo14 { field1: 0, field2: 0 };
        let _ = &foo14.field1;
    }

    use foo20::bar;
    foo20::bar();
    
    foo21::<usize>();
}