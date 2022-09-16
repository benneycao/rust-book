use std::ptr::null;

#[test]
#[allow(unused)]
fn test_reference_as_pointer() {
    let a = 42;
    let a_ptr = &a as *const i32;
    let a_ptr = a_ptr as *mut i32;
    unsafe {
        *a_ptr = 1000;
    }
    println!("a: {} a_ptr: {:p}", a, a_ptr);
}

#[test]
#[allow(unused)]
fn test_transmute() {
    let a: i64 = 42;
    let a_ptr = &a as *const i64;
    let a_addr: usize = unsafe { std::mem::transmute(a_ptr) };
    println!("a: {} a_ptr address: {:p} a_addr: {}", a, a_ptr, a_addr);
}

#[test]
fn test_more_pointer_point_to_same_address() {
    let a = 42;
    let x: Vec<i32>;
    let a_ptr = &a as *const i32;
    let a_ptr = a_ptr as *mut i32;
    let b_ptr = a_ptr;
    unsafe {
        *a_ptr = 1000;
        *b_ptr = 100;
    }
    println!("a: {} a_ptr: {:p} b_ptr: {:p}", a, a_ptr, b_ptr);
}
