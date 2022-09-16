use std::rc::Rc;

struct SharedValue {
    value: i32,
}
#[test]
#[allow(unused)]
fn test_shared_value() {
    let shared_value = Rc::new(SharedValue { value: 42 });
    let owner1 = shared_value.clone();
    let owner2 = shared_value.clone();
    eprintln!("value: {:?} {:?}", owner1.value, owner2.value); // 42 42
    eprintln!("address:  {:p} {:p}", &owner1.value, &owner2.value); // address:  0x600001954030 0x600001954030
}
