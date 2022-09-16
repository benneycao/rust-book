use std::mem::size_of;

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

#[test]
#[allow(unused)]
fn test_array_address() {
    let a = 42;
    let b = &B;
    let c = &C;
    println!("a:{:p} b:{:p} c:{:p}", &a, b, c);
}

#[test]
#[allow(unused)]
fn test_address() {
    let a = 42_usize;
    let b = &B;
    let c = Box::new(C);
    println!("a (an unsigned integer): ");
    println!("  location: {:p}", &a);
    println!("  size:     {}", size_of::<usize>());
    println!("  value:    {}", a);
    println!();

    println!("b (a reference to B): ");
    println!("  location: {:p}", &b);
    println!("  size:     {}", size_of::<&[u8; 10]>());
    println!("  point to: {:p}", b);
    println!("  value:    {:?}", b);
    println!();

    println!("c (a box for C):");
    println!("  location: {:p}", &c);
    println!("  size:     {}", size_of::<Box<Box<[u8; 11]>>>());
    println!("  point to: {:p}", c); // 指向堆中的地址
    println!("  value:    {:?}", c);
    println!();

    println!("B (an array of [u8;10]): ");
    println!("  location: {:p}", &B);
    println!("  size:     {}", size_of::<[u8; 10]>());
    println!("  value:    {:?}", B);
    println!();

    println!("C (an array of [u8;11]): ");
    println!("  location: {:p}", &C);
    println!("  size:     {}", size_of::<[u8; 11]>());
    println!("  value:    {:?}", C);
    println!();
}
