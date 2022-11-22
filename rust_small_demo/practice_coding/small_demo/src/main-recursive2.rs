use crate::List::{Cons, Nil};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Weak<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Weak<List>>> {
        match self {
            Cons(_, items) => Some(items),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Weak::new())));
    println!(
        "1, a strong count ={}, weak count = {}",
        Rc::strong_count(&a),
        Rc::weak_count(&a)
    );
    println!("1, a.tail() = {:?}", a.tail());

    let b = Rc::new(Cons(6, RefCell::new(Rc::downgrade(&a))));
    println!(
        "2, a strong count ={}, weak count = {}",
        Rc::strong_count(&a),
        Rc::weak_count(&a)
    );
    println!(
        "2, b strong count ={}, weak count = {}",
        Rc::strong_count(&b),
        Rc::weak_count(&b)
    );
    println!("2, b.tail() = {:?}", b.tail());

    if let Some(item) = a.tail() {
        *item.borrow_mut() = Rc::downgrade(&b);
    }
    println!(
        "3, a strong count ={}, weak count = {}",
        Rc::strong_count(&a),
        Rc::weak_count(&a)
    );
    println!(
        "3, b strong count ={}, weak count = {}",
        Rc::strong_count(&b),
        Rc::weak_count(&b)
    );
    println!("3, a.tail() = {:?}", a.tail());
}
