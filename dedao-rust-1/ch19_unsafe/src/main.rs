use std::{mem, ptr};

/// unsafe 可以修饰 fn impl trait block
fn main() {
    let x: i32 = 1_i32;
    let mut y: u32 = 1;
    let raw_mut = &mut y as *mut u32 as *mut i32 as *mut i64;
    unsafe {
        *raw_mut = -1;
    }
    println!("{:X} {:X}", x, y);
}

fn raw_to_ref<'a>(p: *const i32) -> &'a i32 {
    unsafe {
        &*p
    }
}

#[test]
#[allow(unused)]
fn test_raw_to_ref() {
    // null ptr
    let p: &i32 = raw_to_ref(std::ptr::null::<i32>());
    println!("{}", p)
}

fn raw_to_ref2<'a>(p: *const i32) -> Option<&'a i32> {
    if p.is_null() {
        None
    } else {
        unsafe { Some(&*p) }
    }
}

#[test]
#[allow(unused)]
fn test_raw_to_ref2() {
    let p = raw_to_ref2(std::ptr::null::<i32>());
    eprintln!("p = {:?}", p);
}

#[test]
#[allow(unused)]
fn test_transmute_copy() {
    let x = vec![1, 2, 3];
    unsafe {
        let t: (usize, usize, usize) = std::mem::transmute_copy(&x);
        println!("{} {} {}", t.0, t.1, t.2);
    }
}

fn swap<T>(x: &mut T, y: &mut T) {
    unsafe {
        let mut t: T = mem::uninitialized();
        ptr::copy_nonoverlapping(&*x, &mut t, 1);
        ptr::copy_nonoverlapping(&*y, x, 1);
        ptr::copy_nonoverlapping(&t, y, 1);
        mem::forget(t);
    }
}


#[test]
#[allow(unused)]
fn test_swap() {
    let mut x = 1;
    let mut y = 100;
    swap(&mut x, &mut y);
    println!("{} {}", x, y);
}

struct Foo {
    a: i32,
    b: i32,
    c: i32,
}

#[test]
#[allow(unused)]
fn test_alias_and_mutation() {
    let mut x = Foo { a: 0, b: 0, c: 0 };
    let pa = &mut x.a;
    let pb = &mut x.b;
    let pc = &x.c;
    *pb += 1;
    let pc2 = &x.c;
    *pa += 10;
    println!("{} {} {} {}", pa, pb, pc, pc2);

    let mut x = vec![1, 2, 3];
    {
        let (first, rest) = x.split_at_mut(1);
        let (second, third) = rest.split_at_mut(1);
        first[0] += 2;
        second[0] += 4;
        third[0] += 8;
        println!("{:?} {:?} {:?}", first, second, third);
    }
    eprintln!("&x = {:?}", &x);
}



















