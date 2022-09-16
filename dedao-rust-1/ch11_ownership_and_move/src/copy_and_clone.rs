#[derive(Copy, Clone, Debug)]
struct Foo(i32);

#[test]
fn test_foo() {
    let p1 = Foo(100);
    let p2 = p1;
    eprintln!("p1 = {:?}", p1);
    eprintln!("p2 = {:?}", p2);
    eprintln!("p1 address = {:p}", &p1);
    eprintln!("p2 address = {:p}", &p2);
}

#[derive(Debug)]
struct Foo1 {
    data: i32,
}

impl Clone for Foo1 {
    fn clone(&self) -> Self {
        Self { data: self.data }
    }
}
