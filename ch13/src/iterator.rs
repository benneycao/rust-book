#[allow(unused)]
fn quick_start_iterator() {
    let v1 = vec![1, 2, 3];
    let v1_iter: Vec<i32> = v1.iter().map(|x| x + x).collect();
    eprintln!("v1_iter = {:?}", v1_iter);
}

#[test]
fn test_quick_start_iterator() {
    quick_start_iterator();
}

#[derive(PartialEq, Debug)]
struct Shoe{
    size: u8,
    color: String
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u8) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|x| x.size == shoe_size)
        .collect()
}

#[test]
fn test_shoes_in_my_size() {
    let shoes = vec![
        Shoe{ size: 44, color: "red".to_string() },
        Shoe{ size: 42, color: "yellow".to_string() },
        Shoe{ size: 43, color: "blue".to_string() },
    ];
    // eprintln!("my_size_show = {:?}", shoes_in_my_size(&shoes, 42));
    assert_eq!(shoes_in_my_size(shoes, 42), vec![Shoe { size: 42, color: "yellow".to_string() }]);
}