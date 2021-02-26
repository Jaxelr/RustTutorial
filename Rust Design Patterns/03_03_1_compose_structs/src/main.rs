#![allow(dead_code)]

struct A {
    f1: u32,
    f2: u32,
    f3: u32,
}

fn foo(a: &mut A) -> &u32 { &a.f2 }
fn bar(a: &mut A) -> u32 { a.f1 + a.f3 }

fn baz(a: &mut A) {
    // The later usage of x causes a to be borrowed for the rest of the function.
    let x = foo(a);
    // Borrow checker error:
    //let y = bar(a); // ~ ERROR: cannot borrow `*a` as mutable more than once
                       //          at a time
    println!("Field 2 {}", x);
}

// A is now composed of two structs - B and C.
struct ABC {
    b: B2,
    c: C2,
}
struct B2 {
    f2: u32,
}
struct C2 {
    f1: u32,
    f3: u32,
}


// These functions take a B or C, rather than A.
fn foo2(b: &mut B2) -> &u32 { &b.f2 }
fn bar2(c: &mut C2) -> u32 { c.f1 + c.f3 }

fn baz2(a: &mut ABC) {
    let x = foo2(&mut a.b);
    // Now it's OK!
    let y = bar2(&mut a.c);
    println!("Field 2 {}", x);
    println!("Sum {}", y);
}

fn main() {
    let mut a = A { f1 : 1, f2 : 2, f3 : 3 };
    baz(&mut a);
    let mut ab = ABC {
        b : B2 {
            f2 : 1
        },
        c : C2 {
            f1 : 2,
            f3 : 3
        }
    };

    baz2(&mut ab);
}
