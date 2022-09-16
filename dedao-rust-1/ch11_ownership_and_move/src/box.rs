#[allow(unused)]
#[derive(Debug)]
struct Foo {
    data: i32,
}
#[test]
fn test_box() {
    let p = Box::new(Foo { data: 100 });
    let p1 = p;
    eprintln!("p1 = {:?}", p1);
}
