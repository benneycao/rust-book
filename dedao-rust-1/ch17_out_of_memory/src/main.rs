use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("Hello, world!");
}

// 内存泄漏
struct Node {
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new() -> Self {
        Self { next: None }
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        eprintln!("drop");
    }
}

#[test]
#[allow(unused)]
fn test_node() {
    let node1 = Rc::new(RefCell::new(Node::new()));
    let node2 = Rc::new(RefCell::new(Node::new()));
    let node3 = Rc::new(RefCell::new(Node::new()));
    {
        node1.borrow_mut().next = Some(node2.clone());
        node2.borrow_mut().next = Some(node3.clone());
        node3.borrow_mut().next = Some(node1.clone());
    }
    eprintln!("hello");
}







































