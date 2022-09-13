use std::fs::File;
use std::io::Read;

#[allow(unused)]
struct Foo {
    data: i32,
}

impl Drop for Foo {
    fn drop(&mut self) {
        eprintln!("this {:p} memory is free  ", &self);
    }
}

#[test]
#[allow(unused)]
fn test_foo() {
    {
        let f = Foo { data: 100 };
        drop(f); // this 0x16d5b66b0 memory is free  1
        println!("foo"); // 2
    }
    println!("Hello"); // 3
}

#[test]
#[allow(unused)]
fn test_file() {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(f) => {
            f
        }
        Err(_) => {
            File::create("hello.txt").unwrap()
        }
    };
    let mut content = String::new();
    let result = f.read_to_string(&mut content);
    println!("{}", result.unwrap());
}