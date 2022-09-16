use std::mem::size_of;

///```rust
/// struct A {
///     a: u8,
///     c: u16,
///     b: u32
/// } =>
/// struct A {
///     a: u8,          -> 1
///     _a: [u8;1],     -> 1
///     c: u16,         -> 2
///     b: u32,         -> 4
/// }
///
/// ```
///
struct A {
    a: u8,
    b: u32,
    c: u16,
}

struct B {
    a: u16,
    b: u32,
}

struct C {
    b: u32,
    a: u16,
}

struct D {
    c: i32,
    a: i64,
    b: u64,
}

struct E {
    a: i32,
    b: u16,
    c: u8,
}

#[test]
#[allow(unused)]
fn test_a() {
    let i = size_of::<A>();
    eprintln!("struct A size = {:?}", i);
    let i = size_of::<B>();
    eprintln!("struct B size = {:?}", i);
    let i = size_of::<C>();
    eprintln!("struct C size = {:?}", i);
    let i = size_of::<D>();
    eprintln!("struct D size = {:?}", i);
    let i = size_of::<E>();
    eprintln!("struct E size = {:?}", i);
}
