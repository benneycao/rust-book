extern crate core;

use std::env::Args;
use std::marker::PhantomData;

fn main() {
    println!("hello");
}


#[test]
#[allow(unused)]
fn test_fn_once() {
    let v: Vec<i32> = vec![1, 2, 3];
    let c = || drop(v);
    c();
    // c();
    println!("following code is upper code unfolding");
    struct ClosureEnvironment {
        _v: Vec<i32>,
    }
    impl ClosureEnvironment {
        fn call_once(self) {
            drop(self._v)
        }
    }
    let v = vec![1, 2, 3];
    let c = ClosureEnvironment { _v: v };
    c.call_once();
    // c.call_once();// error
}

#[test]
#[allow(unused)]
fn test_fn() {
    let v = vec![1, 2, 3];
    let c = |a: i32| for i in &v { println!("{}", *i + a); };
    c(1);
    println!("following code is upper code unfolding");
    struct ClosureEnvironment<'a> {
        _v: &'a Vec<i32>,
    }
    impl ClosureEnvironment<'_> {
        fn call(&self, a: i32) {
            for i in self._v {
                println!("{}", *i + a);
            }
        }
    }
    let v = vec![1, 2, 3];
    let c = ClosureEnvironment { _v: &v };
    c.call(1);
}

#[test]
#[allow(unused)]
fn test_fn_mut() {
    let mut v = vec![1, 2, 3];
    let mut c = |i| v.push(i);
    c(4);
    c(5);
    eprintln!("v = {:?}", v);
    println!("following code is upper code unfolding");
    struct ClosureEnvironment<'a> {
        _v: &'a mut Vec<i32>,
    }
    impl ClosureEnvironment<'_> {
        fn call_mut(&mut self, i: i32) {
            self._v.push(i);
        }
    }
    let mut v = vec![1, 2, 3];
    let mut c = ClosureEnvironment { _v: &mut v };
    c.call_mut(4);
    c.call_mut(5);
    eprintln!("v = {:?}", v);
}





















