use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

#[test]
fn test_mutex_quick_start() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    eprintln!("m = {:?}", m);
}
/*
error
#[test]
fn test_mutex_thread() {
    let counter = Mutex::new(0);
    let mut handles = vec![];
    for _ in 0..10 {
        let handle = thread::spawn(move || {
            // Here counter is moved, so next loop can use it.
            let mut num = counter.lock().unwrap();
            *num +=1;
        });
        handles.push(handle);
    };
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}*/

/*
// also error
#[test]
fn test_mutex_thread() {
    // Rc<T> cannot be sent between threads safely
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}*/


#[test]
fn test_mutex_thread_by_arc() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}


















