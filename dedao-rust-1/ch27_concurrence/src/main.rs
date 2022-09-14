use std::borrow::BorrowMut;
use std::sync::Arc;
use std::thread;

fn main() {
    let mut v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        for i in 4..10 {
            v.push(i);
        }
        eprintln!("v = {:?}", v);
    });
    println!("world");
    handle.join().unwrap();
}
