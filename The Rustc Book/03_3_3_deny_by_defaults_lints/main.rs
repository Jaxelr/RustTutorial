//Note, this compilation has a bunch of errors
//Just looking at the linter options for learning purposes


#![allow(dead_code)]
#![allow(unused_must_use)]
#![allow(unused_variables)]
#![allow(unused_macros)]
#![allow(unused_attributes)]

#![allow(ambiguous_associated_items)] // Example1
#![allow(arithmetic_overflow)] // Example2
#![allow(conflicting_repr_hints)] // Example3
#![allow(const_err)] //Example4
#![allow(ill_formed_attribute_input)] //Example5
#![allow(incomplete_include)] //Example6
#![allow(invalid_type_param_default)] //Example7
#![allow(macro_expanded_macro_exports_accessed_by_absolute_paths)] //Example8
#![allow(missing_fragment_specifier)] //Example9
#![allow(mutable_transmutes)] //Example10
#![allow(no_mangle_const_items)] //Example11
#![allow(order_dependent_trait_objects)] //Examlpe12
#![allow(overflowing_literals)] //Example13
#![allow(patterns_in_fns_without_body)] //Example14
#![allow(pub_use_of_private_extern_crate)] //Example15
#![allow(soft_unstable)] //Example16
#![allow(unconditional_panic)] //Example17
#![allow(unknown_crate_types)] //Example 18
#![crate_type="lol"]

//Example1
enum EnumExample1 {
    V
}

trait TraitExample1 {
    type Example1;
    fn example1() -> Self::Example1;
}

impl TraitExample1 for EnumExample1 {
    type Example1 = u8;
    // `Self::V` is ambiguous because it may refer to the associated type or
    // the enum variant.
    fn example1() -> Self::Example1 { 0 }
}

//Example2
fn example2() {
    1_i32 << 32;
}

//Example3
#[repr(u32, u64)]
enum Example3 {
    Variant1,
}

//Example4
fn example4() {
    #![allow(unconditional_panic)]
    let x: &'static i32 = &(1 / 0);
}

//Example5
#[inline = "this is not valid"]
fn foo() {}

//Example6
fn example6() {
    include!("foo.txt");
}

//Example7
fn example7<T=i32>(t: T) {}

//Example8
macro_rules! define_exported {
    () => {
        #[macro_export]
        macro_rules! exported {
            () => {};
        }
 
 
    };
}

define_exported!();

//Example9
macro_rules! example9 {
    ($e) => {}
 }

//Example10
fn example10() {
    unsafe {
        let y = std::mem::transmute::<&i32, &mut i32>(&5);
    }
}

//Example11
#[no_mangle]
const FOO: i32 = 5;

//Example12
pub trait ExampleTrait12 {}

impl ExampleTrait12 for dyn Send + Sync { }
impl ExampleTrait12 for dyn Sync + Send { }

//Example13
fn example13() {
    let x: u8 = 1000;
}

//Example14
trait ExampleTrait14 {
    fn example14(mut arg: u8);
}

//Example15
extern crate core;
pub use core as reexported_core;

//Example16
#[cfg(test)]
extern crate test;

#[bench]
fn example16(b: &mut test::Bencher) {
    b.iter(|| 123)
}

//Example17 
fn example17() {
    let x = 1 / 0;
}

//Example18

fn main() {
    example2();
    example4();
    crate::exported!();
    example17();
}