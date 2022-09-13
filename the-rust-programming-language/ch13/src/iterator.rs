#[allow(unused)]
fn quick_start_iterator() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter().map(|x| x + x).collect::<Vec<i32>>();
    eprintln!("v1_iter = {:?}", v1_iter);
}

#[test]
fn test_quick_start_iterator() {
    quick_start_iterator();
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u8,
    color: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u8) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|x| x.size == shoe_size)
        .collect()
}

#[test]
fn test_shoes_in_my_size() {
    let shoes = vec![
        Shoe { size: 44, color: "red".to_string() },
        Shoe { size: 42, color: "yellow".to_string() },
        Shoe { size: 43, color: "blue".to_string() },
    ];
    // eprintln!("my_size_show = {:?}", shoes_in_my_size(&shoes, 42));
    assert_eq!(shoes_in_my_size(shoes, 42), vec![Shoe { size: 42, color: "yellow".to_string() }]);
}

/// Create my struct implement the Iterator trait
struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}


impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn test_counter() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
