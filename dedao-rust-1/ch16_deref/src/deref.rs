use std::ops::Deref;
use std::rc::Rc;

#[test]
#[allow(unused)]
fn test_deref() {
    let x = String::from("hello");
    let y = &*x;
    let b = Box::new(100);
    let p = b.deref(); // &i32
    let p = *b; // i32
    let p = &*b; // &i32
    let v = vec![1, 2, 3];
    let p = &v[0..1];
    eprintln!("p = {:?}", p);
}

#[test]
#[allow(unused)]
fn test_auto_deref() {
    let s = "hello";
    let x = s.deref();
    eprintln!("length = {:?}", s.len());
    eprintln!("length = {:?}", (&s).len());

    let s = Rc::new(String::from("hello world"));
    eprintln!("length = {:?}", s.len());
    eprintln!("length = {:?}", s.deref().len());
    eprintln!("length = {:?}", s.deref().deref().len());
    eprintln!("length = {:?}", (*s).len());
}

#[test]
fn test_conflicts_deref() {
    let s = Rc::new(String::from("hello"));
    let s1 = s.clone();
    let ps1 = (*s).clone();
    let pps1 = (&**s).clone();
}
