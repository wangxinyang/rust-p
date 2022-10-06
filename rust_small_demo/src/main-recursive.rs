use crate::List::{Cons, Nil};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
// enum List {
//     Cons(Rc<RefCell<i32>>, Rc<RefCell<List>>),
//     Nil,
// }
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(3, RefCell::new(Rc::new(Nil))));
    println!("a is {:?}", a);
    println!("1, a rc count = {}", Rc::strong_count(&a));
    println!("1, a tail = {:?}", a.tail());

    let b = Rc::new(Cons(6, RefCell::new(Rc::clone(&a))));
    println!("b is {:?}", b);
    println!("2, a rc count = {}", Rc::strong_count(&a));
    println!("2, b rc count = {}", Rc::strong_count(&b));
    println!("2, b tail = {:?}", b.tail());

    // let value = Rc::new(RefCell::new(5));
    // let a = Rc::new(Cons(Rc::clone(&value), Rc::new(RefCell::new(Nil))));
    // println!(" a before is {:?}", a);
    // *value.borrow_mut() = 100;
    // println!(" a after is {:?}", a);
    if let Some(value) = a.tail() {
        *value.borrow_mut() = Rc::clone(&b);
    }
    // println!("a is {:?}", a);
    println!("3, a rc count = {}", Rc::strong_count(&a));
    println!("3, b rc count = {}", Rc::strong_count(&b));
    println!("3, a tail = {:?}", a.tail());
}
