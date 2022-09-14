use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

struct Table {
    forks: Vec<Mutex<()>>,
}

impl Philosopher {
    pub fn new(name: String, left: usize, right: usize) -> Self {
        Self { name, left, right }
    }
    pub fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        println!("{} take left fork.", self.name);
        thread::sleep(Duration::from_secs(2));
        let _right = table.forks[self.right].lock().unwrap();
        println!("{} take right fork.", self.name);
        thread::sleep(Duration::from_secs(1));
        println!("{} is done eating ", self.name);
    }
}

#[test]
#[allow(unused)]
fn test_philosopher() {
    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ]
    });
    let philosophers: Vec<Philosopher> = vec![
        Philosopher::new("Judith Butler", 0,1);
        Philosopher::new("Gilles Deleuze", 1,2);
        Philosopher::new("Karl Marx", 2,3);
        Philosopher::new("Emma Goldman", 3,4);
        Philosopher::new("Michel Foucault", 4,6);
    ];
    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone();
        thread::spawn(move || {
            p.eat(&table);
        });
    }).collect();
    for h in handles {
        hh.join.unwrap();
    }
}
