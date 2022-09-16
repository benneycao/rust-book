use std::thread;
use std::time::Duration;

#[test]
fn test_thread() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("spawned thread number {} ", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..6 {
        println!("main thread number {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

#[test]
fn test_thread_spawn_join() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("spawned thread number {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // if you move the join function to here it will not running the following code,
    // because the caller thread is blocking with join function it will await the caller thead running end.
    // handle.join().unwrap();
    for i in 0..5 {
        println!("main thread number {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    // call join function it'll blocking the caller thread. it'll await the caller thread running end;
    handle.join().unwrap();
}

#[test]
fn test_closure_get_ownership() {
    let v = vec![1, 2, 3];

    // pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    // where
    //     F: FnOnce() -> T

    // not to be allowed because spawn function need a FnOnce trait, but now we send a Fn trait, can not be allowed compile
    // let handle = thread::spawn(|| {
    //     println!("Here's a vector: {:?}", v);
    // });

    // Here we adding a move keyword, move the vector ownership to the another thread, change the Fn to FnOnce.
    // And we cannot use the v variable, because of the v variable's ownership is moved to the spawned thread
    let handle = thread::spawn(move || eprintln!("Here's a vector = {:?}", v));

    // error println("{:?}", v); it'll say value used here after move

    handle.join().unwrap();
}
