mod r#box;
mod copy_and_clone;
mod drop;

#[derive(Debug)]
struct Foo {
    data: i32,
}

impl Clone for Foo {
    fn clone(&self) -> Self {
        Self { data: self.data }
    }
}

impl Copy for Foo {}

fn main() {
    let mut s = String::from("hello");
    s.push_str(" world");
    println!("{s}");

    let v1 = Foo { data: 100 };
    let v2 = v1;
    eprintln!("v1 = {:?}", v1);
    eprintln!("v2 = {:?}", v2);
    // 0x16d53ef1c - 0x16d53ef18 = 4;
    println!("the v1 address: {:p}", &v1); // the v1 address: 0x16d53ef18
    println!("the v2 address: {:p}", &v2);  // the v2 address: 0x16d53ef1c
}

