fn heap() {
    let a = 10;
    let b = Box::new(20);
    println!("{} + {} = {}", a, b, a + *b);
}

#[test]
#[allow(unused)]
fn test_heap() {
    heap();
}

fn heap_2() {
    let a = Box::new(1);
    let b = Box::new(1);
    let c = Box::new(1);
    let result = *a + *b + *c;
    drop(a);
    let d = Box::new(1);
    let result2 = *b + *c + *d;
    println!("{} {}", result, result2);
}

#[test]
#[allow(unused)]
fn test_heap_2() {
    heap_2();
}
