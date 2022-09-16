use crate::r#box::List::{Cons, Nil};
use std::io::Take;
use std::ops::Deref;

fn quick_start() {
    let s = Box::new("Hello");
    eprintln!("s = {:?}", s);
}

#[test]
fn test_quick_start() {
    quick_start();
}

// enum List {
//     Cons(i32, List),
//     Nil
// }
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn box_list() {
    // cannot be allowed to compile,
    // because the compiler doesn't know the variable size.
    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    eprintln!("list = {:?}", list);
}

#[test]
fn test_box_list() {
    box_list();
}

#[test]
fn test_deref() {
    let x = 5;
    // generic reference
    let y = &x;
    assert_eq!(5, *y);
    assert_eq!(5, x);

    // use box like generic reference.
    let y = Box::new(x);
    assert_eq!(5, *y);
    assert_eq!(5, x);
}

/// Defining our own smart pointer
#[test]
fn owner_smart_pointer() {
    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    /// impl Deref trait for to auto dereference variables
    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    /// # deref coercions
    // deref coercions, if the function receive a reference type when
    // you send a type but not match the function reference type,
    // it will auto dereference if the result is matched the function need also can run.
    // If can not auto dereference or auto dereference's result is also not match the function need it'll compile fail.
    struct MyI32(i32);
    impl Deref for MyI32 {
        type Target = i32;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    fn show_i32(i: &i32) {
        println!("{i}")
    }
    let i = MyI32(100);
    show_i32(&i);
}
