use std::cell::{Cell, RefCell};
use std::rc::Rc;
use crate::ref_cell::List::{Cons, Nil};

#[test]
fn test_cell() {
    let x = Cell::new(100);
    let i = x.get();
    eprintln!("i = {:?}", i);
    x.set(10000);
    let i = x.get();
    eprintln!("i = {:?}", i);
    eprintln!("x = {:?}", x);
}

#[test]
fn test_ref_cell() {
    let x = RefCell::new("hell".to_string());
    x.borrow_mut().push_str("o");
    eprintln!("x = {:?}", x);
}

/// Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

#[test]
fn test_list() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}



















