fn quick_start_iterator() {
    let v1 = vec![1, 2, 3];
    let v1_iter: Vec<i32> = v1.iter().map(|x| x + x).collect();
    eprintln!("v1_iter = {:?}", v1_iter);
}

#[test]
fn test_quick_start_iterator() {
    quick_start_iterator();
}