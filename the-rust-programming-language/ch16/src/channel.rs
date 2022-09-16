use std::borrow::Borrow;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// create a channel by mpsc's channel function `mpsc::channel()`.
/// mpsc means multiple producer single consumer.
#[test]
fn test_channel() {
    // multiple producer, single consumer => mpsc
    let (sx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        sx.send(val).unwrap();
        // if you want to use the value after it sent, it'll not to be allowed to compile.
        // Because the send function will get the parameter's ownership.
        //pub fn send(&self, t: T) -> Result<(), SendError<T>> {
        // eprintln!("val = {:?}", val); error
    });
    let received = rx.recv().unwrap();
    eprintln!("Got: {:?}", received);
}

#[test]
fn test_send_many_value() {
    let (sx, rx) = mpsc::channel();
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("main"),
        String::from("thread"),
    ];
    thread::spawn(move || {
        for val in vals {
            sx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        eprintln!("Got = {:?}", received);
    }
}

/// Create multiple producer/sender by clone() function.
#[test]
fn test_multiple_producer() {
    let (sx, rx) = mpsc::channel();
    let sx1 = sx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            sx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            sx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        eprintln!("Got = {:?}", received);
    }
}
