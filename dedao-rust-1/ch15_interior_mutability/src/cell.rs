use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[test]
#[allow(unused)]
fn test_rc() {
    let r1 = Rc::new(1);
    println!("reference count {}", Rc::strong_count(&r1)); // 1
    let r2 = r1.clone();
    println!("reference count {}", Rc::strong_count(&r2)); // 2
}

#[test]
#[allow(unused)]
fn test_cell() {
    let data = Cell::new(100);
    let p = &data;
    data.set(10);
    eprintln!("data inner value is {}", p.get());

    p.set(1000);
    eprintln!("data value is {:?}", data);
}

#[test]
#[allow(unused)]
fn test_refcell() {
    let shared_vec = RefCell::new(vec![1, 2, 3]);
    let shared1 = &shared_vec;
    let shared2 = &shared1;
    shared1.borrow_mut().push(4);
    eprintln!("vec = {:?}", shared_vec.borrow());

    shared2.borrow_mut().push(5);
    eprintln!("vec = {:?}", shared_vec.borrow());

}



















