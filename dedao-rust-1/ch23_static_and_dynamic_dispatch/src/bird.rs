trait Bird {
    fn fly(&self);
}

struct Duck;

struct Swan;

impl Bird for Duck {
    fn fly(&self) {
        println!("duck duck");
    }
}

impl Bird for Swan {
    fn fly(&self) {
        println!("swan swan");
    }
}

// 通过 impl Trait 实现静态分派
fn static_dispatch_fly1(bird: &impl Bird) {
    bird.fly();
}

/// 通过泛型实现静态分派
fn static_dispatch_fly2<T: Bird>(t: &T) {
    t.fly();
}

/// 实现动态分派
fn dynamic_dispatch_fly(bird: Box<&dyn Bird>) {
    bird.fly();
}

fn create_bird(name: &str) -> Option<Box<dyn Bird>> {
    match name {
        "duck" => { Some(Box::new(Duck)) }
        "swan" => { Some(Box::new(Swan)) }
        _ => { None }
    }
}

#[test]
#[allow(unused)]
fn test_bird() {
    let duck = Duck;
    let swan = Swan;
    println!("static_dispatch_fly1(bird: &impl Bird)");
    static_dispatch_fly1(&duck);
    static_dispatch_fly1(&swan);
    println!();

    println!("static_dispatch_fly2<T: Bird>(t: &T)");
    static_dispatch_fly2(&duck);
    static_dispatch_fly2(&swan);
    println!();

    println!("dynamic_dispatch_fly(bird: Box<&dyn Bird>)");
    dynamic_dispatch_fly(Box::new(&duck));
    dynamic_dispatch_fly(Box::new(&swan));
    println!();


    println!("create_bird(name) -> Option<Box<dyn Bird>>");
    let bird = create_bird("duck");
    match bird {
        None => {}
        Some(b) => {
            b.fly();
        }
    }
}