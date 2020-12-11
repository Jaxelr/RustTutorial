//Requires unstable channel
//#![feature(asm)]  //Example2
//#![feature(non_ascii_idents)]  //Example8 //Example23 //Example39
//#![feature(generic_associated_types)] //Example17
//#![feature(no_sanitize)] //Example19

//All these cases are warning by default.

#![allow(array_into_iter)] //Example1
#![allow(bare_trait_objects)] //Example3
#![allow(bindings_with_variant_name)] //Example4
#![allow(cenum_impl_drop_cast)] //Example5
#![allow(clashing_extern_declarations)] //Example6
#![allow(coherence_leak_check)] //Example7
#![allow(confusable_idents)] //Example8
#![allow(const_evaluatable_unchecked)] //Example9
#![allow(const_item_mutation)] //Example10
#![allow(dead_code)] //Example11
#![allow(deprecated)] //Example12
#![allow(ellipsis_inclusive_range_patterns)] //Example13
#![allow(illegal_floating_point_literal_pattern)] //Example14
#![allow(improper_ctypes)] //Example15
#![allow(improper_ctypes_definitions)] //Example16
#![allow(indirect_structural_match)] //Example18
#![allow(invalid_value)] //Example20
#![allow(irrefutable_let_patterns)] //Example21
#![allow(late_bound_lifetime_arguments)] //Example22
#![allow(mutable_borrow_reservation_conflict)] //Example24
#![allow(no_mangle_generic_items)] //Example25
#![allow(non_camel_case_types)] //Example26
#![allow(non_shorthand_field_patterns)] //Example27
#![allow(non_snake_case)] //Example28
#![allow(non_upper_case_globals)] //Example29
#![allow(nontrivial_structural_match)] //Example30
#![allow(overlapping_patterns)] //Example31
#![allow(path_statements)] //Example32
#![allow(private_in_public)] //Example33
#![allow(redundant_semicolons)] //Example34
#![allow(renamed_and_removed_lints)] //Example35
#![deny(raw_pointer_derive)] //Example35
#![allow(safe_packed_borrows)] //Example36
#![allow(type_alias_bounds)] //Example37
#![allow(tyvar_behind_raw_pointer)] //Example38
#![allow(unconditional_recursion)] //Example40
#![allow(unreachable_code)] //Example41
#![allow(unreachable_patterns)] //Example42
#![allow(unstable_name_collisions)] //Example43
#![allow(unused_assignments)] //Example44
#![allow(unused_attributes)] //Example45
#![macro_export] //Example45
#![allow(unused_braces)] //Example46
#![allow(unused_comparisons)] //Example47
#![allow(unused_imports)] //Example48
#![allow(unused_labels)] //Example49
#![allow(unused_macros)] //Example50
#![allow(unused_must_use)] //Example51
#![allow(unused_mut)] //Example52
#![allow(unused_parens)] //EXample53
#![allow(unused_unsafe)] //Example54
#![allow(unused_variables)] //Example55

//Example1
fn example1() {
    [1, 2, 3].into_iter().for_each(|n| { *n; }); //use iter() instead of into_iter()
}

//Example3
trait Trait { }

fn takes_trait_object(_: Box<Trait>) {
}

//Example4
pub enum Example4 {
    Foo,
    Bar,
}

pub fn example4(x: Example4) { //These matches should have a diff name
    match x {
        Foo => {}
        Bar => {}
    }
}

//Example5
enum Example5 {
    A,
}

impl Drop for Example5 {
    fn drop(&mut self) {
        println!("Drop");
    }
}

//Example6
mod m {
    extern "C" {
        fn example6();
    }
}

extern "C" {
    fn example6(_: u32);
}

//Example7
trait Example7 { }
impl Example7 for for<'a> fn(&'a u8) { }
impl<'a> Example7 for fn(&'a u8) { }

//Example8
// Latin Capital Letter E With Caron
//pub const Ě: i32 = 1;
// Latin Capital Letter E With Breve
//pub const Ĕ: i32 = 2;

//Example9
const fn example9<T>() -> usize {
    if std::mem::size_of::<*mut T>() < 8 { // size of *mut T does not depend on T
        4
    } else {
        8
    }
}

fn test<T>() {
    let _ = [0; example9::<T>()];
}

//Example10
const EXAMPLE10: [i32; 1] = [0];

//Example12
#[deprecated]
fn example12() {}

fn usage_example12() {
    example12();
}

//Example13
fn example13() {
    let x = 123;
    match x {
        0...100 => {}
        _ => {}
    }
}

//Example14
fn example14() {
    let x = 42.0;

    match x {
        5.0 => {}
        _ => {}
    }
}

//Example15
extern "C" {
    static STATIC: String;
}

//Example16
pub extern "C" fn str_type(p: &str) { }

//Example18
struct NoDerive(i32);
impl PartialEq for NoDerive { fn eq(&self, _: &Self) -> bool { false } }
impl Eq for NoDerive { }
#[derive(PartialEq, Eq)]
struct WrapParam<T>(T);
const WRAP_INDIRECT_PARAM: & &WrapParam<NoDerive> = & &WrapParam(NoDerive(0));

//Example19
// #[inline(always)]
// #[no_sanitize(address)]
// fn x() {}

//Example20
fn example20() {
    unsafe {
        let x: &'static i32 = std::mem::zeroed();
    }
}

//Example21
fn example21() {
    if let _ = 123 {
        println!("always runs!");
    }
}

//Example22
struct Example22;

impl Example22 {
    fn late<'a, 'b>(self, _: &'a u8, _: &'b u8) {}
}

fn usage_example22() {
    Example22.late::<'static>(&0,&0);
}

//Example23
// The Japanese katakana character エ can be confused with the Han character 工.
//const エ: &'static str = "アイウ";

//Example24
fn example24() {
    let mut v = vec![0, 1, 2];
    let shared = &v;
    v.push(shared.len());
}

//Example25
#[no_mangle]
fn example25<T>(t: T) {
}

//Example26
struct example_26;

//Example27
struct StructExample27 {
    x: i32,
    y: i32,
}

fn example27() {
    let p = StructExample27 {
        x: 5,
        y: 5,
    };

    match p {
        StructExample27 { x: x, y: y } => (),
    }
}

//Example28
fn example28() {
    let VALUE_EXAMPLE_28 = 5;
}

//Example29
static max_points: i32 = 5;

//Example30
#[derive(Copy, Clone, Debug)]
struct NoDerive2(u32);
impl PartialEq for NoDerive2 { fn eq(&self, _: &Self) -> bool { false } }
impl Eq for NoDerive2 { }

fn example30() {
    const INDEX: Option<NoDerive2> = [None, Some(NoDerive2(10))][0];
    match None { Some(_) => panic!("whoops"), INDEX => dbg!(INDEX), };
}

//Example31
fn example31() {
    let x = 123u8;
    match x {
        0..=100 => { println!("small"); }
        100..=255 => { println!("large"); }
    }
}

//Example32
fn example32() {
    let x = 42;

    x;
}

//Example33
struct StructExample33;

mod ModExample33 {
    struct Priv;
    impl super::StructExample33 {
        pub fn f(_: Priv) {}
    }
}

//Example36
#[repr(packed)]
pub struct Unaligned<T>(pub T);

pub struct StructExample36 {
    start: u8,
    data: Unaligned<u32>,
}

fn example36() {
    let x = StructExample36 { start: 0, data: Unaligned(1) };
    let y = &x.data.0;
}

//Example37
type SendVec<T: Send> = Vec<T>;

//Example38
fn example38() {
    let data = std::ptr::null();
    let _ = &data as *const *const ();

    
    if data.is_null() {}
}

//Example40
fn example40() {
    example40();
}

//Example41
fn example41() {
    panic!("we never go past here!");

    let x = 5;
}

//Example42
fn example42() {
    let x = 5;
    match x {
        y => (),
        5 => (),
    }
}

//Example43
trait MyIterator : Iterator {
    // is_sorted is an unstable method that already exists on the Iterator trait
    fn is_sorted(self) -> bool where Self: Sized {true}
}

impl<T: ?Sized> MyIterator for T where T: Iterator { }

fn example43() {
    let x = vec![1,2,3];
    let _ = x.iter().is_sorted();
}

//Example44
fn example44() {
    let mut x = 5;
    x = 6;
}

//Example46
fn example46() {
    if { true } {
        // ...
    }
}

//Example47
fn example47(x: u8) {
    x >= 0;
}

//Example48
use std::collections::HashMap;

//Example49
fn example49() {
    'unused_label: loop {}
}

//Example50
macro_rules! unused {
    () => {};
}

//Example51
fn example51() -> Result<(), ()> {
    Ok(())
}

//Example52
fn example52() {
    let mut x = 5;
}

//Example53
fn example53() {
    if (true) {

    }
}

//Example54
fn example54() {
    unsafe {}
}

//Example55
fn example55() {
    let x = 5;
}

fn main() {
    //Example2 //Requires unstable channel
    // #[cfg(target_arch="x86_64")]
    // unsafe {
    //     asm!("mov {0}, {0}", in(reg) 0i16);
    // }

    //Example5
    let e = Example5::A;
    let i = e as u32;

    //Example10
    EXAMPLE10[0] = 1;
    // This will print "[0]".
    println!("{:?}", EXAMPLE10);

    //Example18 //Warning this gives a compiler error
    // match WRAP_INDIRECT_PARAM {
    //     WRAP_INDIRECT_PARAM => { }
    //     _ => { }
    // }

    //Example19
    //x()
    
    //Example23
    //const エ: &'static str = "アイウ";

    //Example34
    let _ = 123;;

    //Example39
    //const µ: f64 = 0.000001;

    //Example41
    #[test]
    fn example41() {
        // This test will not fail because it does not run.
        assert_eq!(1, 2);
    }

    example51();
}