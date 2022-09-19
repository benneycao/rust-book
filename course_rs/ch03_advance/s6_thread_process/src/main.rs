use std::sync::Once;
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

fn main() {
    // 创建一个线程A
    let new_thread = thread::spawn(move || {
        // 再创建一个线程B
        thread::spawn(move || {
            loop {
                println!("I am a new thread.");
            }
        })
    });

    // 等待新创建的线程执行完成
    new_thread.join().unwrap();
    println!("Child thread is finish!");

    // 睡眠一段时间，看子线程创建的子线程是否还在运行
    thread::sleep(Duration::from_millis(10000000));
}

#[test]
#[allow(unused)]
fn test_move() {
    let v = vec![1, 2, 3];
    let v1 = Arc::new(vec![2, 3, 4, 5]);
    let arc = v1.clone();
    let handle = thread::spawn(move || {
        eprintln!("v = {:?}", &v);
        eprintln!("arc = {:?}", arc);
    });
    eprintln!("v1 = {:?}", v1);
    handle.join().unwrap();
}

#[test]
#[allow(unused)]
fn test_close_thread() {
    // thread A
    let a = thread::spawn(|| {
        // thread B
        thread::spawn(|| {
            loop {
                println!("I am B");
            }
        })
    });
    // 等待 线程 A 执行完毕
    a.join().unwrap();
    println!("Child thread is finish! ");

    // 睡眠一段时间，看线程创建的子线程是否还在运行
    thread::sleep(Duration::from_millis(100));

}


#[test]
#[allow(unused)]
/// 使用 barrier 让多个线程，都执行到某个点后，才继续一起执行
fn test_thread_barrier() {
    let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));
    for _ in 0..6 {
        let b = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("before wait");
            b.wait();
            println!("after wait");
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

static mut VAL: usize = 0;
static INIT: Once = Once::new();

#[test]
#[allow(unused)]
fn test_once_call_function() {
    let handle1 = thread::spawn(move || {
        INIT.call_once(|| {
            unsafe {
                VAL = 1;
            }
        });
    });

    let handle2 = thread::spawn(move || {
        INIT.call_once(|| {
            unsafe {
                VAL = 2;
            }
        });
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", unsafe { VAL });
}

















