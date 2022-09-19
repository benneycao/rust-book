use std::collections::HashMap;
use std::vec::IntoIter;

#[test]
#[allow(unused)]
fn test_demo() {
    let arr = vec![1, 2, 3];
    for i in &arr {
        println!("{i}");
    }
    println!();
    print!("[");
    for i in 0..arr.len() {
        print!("{}: {},", i, arr[i]);
    }
    println!("]");

    let mut arr_iter = arr.into_iter();
    eprintln!("arr_iter.next() = {:?}", arr_iter.next());
    eprintln!("arr_iter.next() = {:?}", arr_iter.next());
    eprintln!("arr_iter.next() = {:?}", arr_iter.next());
    eprintln!("arr_iter.next() = {:?}", arr_iter.next());
}

#[test]
#[allow(unused)]
fn test_demo1() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
    eprintln!("v1 = {:?}", v1);
}

#[test]
#[allow(unused)]
fn test_demo2() {
    let v1 = vec![1, 2, 3];
    let map: Vec<_> = v1.iter().map(|x| x + 1).collect();
    eprintln!("map = {:?}", map);
}

#[test]
#[allow(unused)]
fn test_demo3() {
    let names = ["cjx", "benny"];
    let ages = [26, 18_u8];
    let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();
    eprintln!("folks = {:?}", folks);
}

struct Shoe {
    size: u32,
    style: String,
}


fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[derive(Default)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Self { ..Default::default() }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else { None }
    }
}

#[test]
#[allow(unused)]
fn test_counter() {
    let mut counter = Counter::new();
    eprintln!("counter.next() = {:?}", counter.next());
    eprintln!("counter.next() = {:?}", counter.next());
    eprintln!("counter.next() = {:?}", counter.next());
    eprintln!("counter.next() = {:?}", counter.next());
    eprintln!("counter.next() = {:?}", counter.next());
    eprintln!("counter.next() = {:?}", counter.next());
}

#[test]
#[allow(unused)]
fn test_other_methods() {
    let counter: u32 = Counter::new().zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    eprintln!("counter = {:?}", counter);
}

#[test]
#[allow(unused)]
fn test_enumerate() {
    let v = vec![1u8, 2, 3, 4, 5, 6, 7];
    for (k, v) in v.iter().enumerate() {
        eprintln!("k = {:?}, v = {:?}", k, v);
    }
    let val = v.iter().enumerate()
        .filter(|(idx, _)| idx % 2 == 0)
        .map(|(idx, val)| val
        )
        .fold(0, |sum, acm| sum + acm);
    eprintln!("val = {:?}", val);
}





















