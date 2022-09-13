use std::rc::Rc;
use crate::rc::List::{Cons, Nil};

/// # Rc<T> Reference counting smart pointer
/// use Rc<T> to sharing data
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

#[test]
fn test_list() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
    eprintln!("a = {:?}", a);
    eprintln!("b = {:?}", b);
    eprintln!("c = {:?}", c);
}