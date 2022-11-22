use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
fn main() {
    let leaf = Rc::new(Node {
        value: 10,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf = {:?}", leaf);
    println!(
        "leaf strong count = {} weak count = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    let branch = Rc::new(Node {
        value: 20,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    println!(
        "branch strong count = {} weak count = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch)
    );
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!(
        "leaf strong count = {} weak count = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
    println!(
        "branch strong count = {} weak count = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch)
    );
}
