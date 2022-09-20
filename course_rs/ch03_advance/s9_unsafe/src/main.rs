use std::slice;
use std::slice::from_raw_parts;
use std::str::from_utf8_unchecked;

fn main() {
    let num = 100;
    let ptr = &num as *const i32;
    unsafe {
        let p = ptr as *mut i32;
        *p = 199;
    }
    eprintln!("num = {:?}", num);
}

struct Location {
    pointer: usize,
    length: usize,
}

fn get_memory_location(s: &str) -> Location {
    let pointer = s.as_ptr() as usize;
    let length = s.len();
    Location { pointer, length }
}

fn get_str_at_location(local: Location) -> &'static str {
    unsafe { from_utf8_unchecked(from_raw_parts(local.pointer as *const u8, local.length)) }
}

#[test]
#[allow(unused)]
fn test_location() {
    let location = get_memory_location("hello");
    let message = get_str_at_location(location);
    eprintln!("location1 = {:?}", message);
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> Option<(&mut [i32], &mut [i32])> {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    if mid <= len {
        unsafe {
            Some((
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            ))
        }
    } else { None }
}

#[test]
#[allow(unused)]
fn test_split_at_mut() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let x = split_at_mut(r, 9);
    match x {
        Some(a) => {
            assert_eq!(a.0, &mut [1, 2, 3]);
            assert_eq!(a.1, &mut [4, 5, 6])
        }
        None => {
            eprintln!("x = {:?}", x);
        }
    }
}

#[test]
#[allow(unused)]
fn test_ffi() {
    extern "C" {
        fn abs(input: i32) -> i32;
    }
    unsafe {
        println!("{}", abs(-3));
    }

}


















