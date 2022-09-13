fn main() {
    let mut v1: Vec<i32> = Vec::new();
    v1.index()
    println!("Start: length {} capacity {} ", v1.len(), v1.capacity());
    for i in 1..10 {
        v1.push(i);
        println!("[Pushed {}] length {} capacity {}", i, v1.len(), v1.capacity());
    }
    let mut v1 = Vec::<i32>::with_capacity(1);
    println!("Start: length {} capacity {} ", v1.len(), v1.capacity());
    v1.reserve(10);
    for i in 1..10 {
        v1.push(i);
        println!("[Pushed {}] length {} capacity {}", i, v1.len(), v1.capacity());
    }
}

struct Foo {
    dropped: bool,
}

impl Foo {
    fn new() -> Self {
        Foo { dropped: false }
    }
}

impl Drop for Foo {
    fn drop(&mut self) {
        self.dropped = true;
    }
}

struct R<'a> {
    inner: Option<&'a Foo>,
}

impl<'a> R<'a> {
    fn new() -> Self{
        R {inner: None}
    }
    fn set_ref(&mut self, ptr: &'a Foo) {
        self.inner = Some(ptr)
    }
}

impl<'a> Drop for R<'a> {
    fn drop(&mut self) {
        if let Some(ref inner) = self.inner {
            println!("dropped R when Foo is {}", inner.dropped);
        }
    }
}

#[test]
fn test_foo() {
    {
        let (a, mut b): (Foo, R) = (Foo::new(), R::new());
        b.set_ref(&a);
    }

    {
        let (mut a, b) = (R::new(), Foo::new());
        a.set_ref(&b);
    }
}



















