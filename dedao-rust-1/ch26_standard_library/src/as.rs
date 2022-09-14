use std::ops::Deref;

struct MyStr {
    buf: Vec<u8>,
}

impl Deref for MyStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        unsafe { std::str::from_utf8_unchecked(&self.buf) }
    }
}

impl AsRef<str> for MyStr {
    fn as_ref(&self) -> &str {
        self
    }
}

#[test]
#[allow(unused)]
fn test_hello() {
    let x = MyStr { buf: vec![104] };
    let x1 = x.as_ref();
    println!("{x1}");
}