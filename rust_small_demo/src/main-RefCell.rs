use crate::List::{Cons, Nil};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    let test = RefCell::new(5);
    *test.borrow_mut() += 1;
    println!("value change {:?}", test);

    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(7)), Rc::clone(&a));

    println!("count {}", Rc::strong_count(&a));
    // println!("count {}", Rc::strong_count(&b));
    // println!("count {}", Rc::strong_count(&c));

    println!("value change before {:?}", a);
    println!("value change before {:?}", b);
    println!("value change before {:?}", c);

    *value.borrow_mut() += 10;

    println!("value change after {:?}", a);
    println!("value change after {:?}", b);
    println!("value change after {:?}", c);
}
