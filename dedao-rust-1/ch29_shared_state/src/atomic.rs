use std::sync::Arc;
use std::sync::atomic::{AtomicIsize, Ordering};
use std::thread;

const COUNT: u32 = 1000000;

#[test]
#[allow(unused)]
fn test_atomic() {
    let global = Arc::new(AtomicIsize::new(0));
    let clone1 = global.clone();
    let thread1 = thread::spawn(move || {
        for _ in 0..COUNT {
            clone1.fetch_add(1, Ordering::SeqCst);
        }
    });
    let clone2 = global.clone();
    let thread2 = thread::spawn(move || {
        for _ in 0..COUNT {
            clone2.fetch_sub(1, Ordering::SeqCst);
        }
    });
    thread1.join().unwrap();
    thread2.join().unwrap();
    eprintln!("final value = {:?}", global);
}

#[test]
#[allow(unused)]
fn test_not_atomic() {
    let global = Arc::new(AtomicIsize::new(0));
    let clone1 = global.clone();
    let thread1 = thread::spawn(move || {
        for _ in 0..COUNT {
            let mut value = clone1.load(Ordering::SeqCst);
            value += 1;
            clone1.store(value, Ordering::SeqCst);
        }
    });
    let clone2 = global.clone();
    let thread2 = thread::spawn(move || {
        for _ in 0..COUNT {
            let mut value = clone2.load(Ordering::SeqCst);
            value -= 1;
            clone2.store(value, Ordering::SeqCst);
        }
    });
    thread1.join().ok();
    thread2.join().ok();
    println!("final value: {:?}", global);
}