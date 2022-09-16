#[test]
fn test_integers() {
    let mut vec = vec![1, 5, 2, 4, 3, 7, 6, 9, 8, 10];
    vec.sort();
    eprintln!("vec = {:?}", vec);
}

#[test]
fn test_float() {
    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];
    let x = 1.1;
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    eprintln!("vec = {:?}", vec);
}

#[test]
fn test_struct() {
    #[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
    struct Person {
        name: String,
        age: u32,
    }

    impl Person {
        pub fn new(name: String, age: u32) -> Self {
            Person { name, age }
        }
    }
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];
    // 根据获得的自然顺序（name 和 age）对 people 进行排序
    people.sort();

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("John".to_string(), 1),
            Person::new("Zoe".to_string(), 25),
        ]
    );

    // 根据 age 值对 people 进行排序
    people.sort_by(|a, b| b.age.cmp(&a.age));

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("Zoe".to_string(), 25),
            Person::new("John".to_string(), 1),
        ]
    );
}
