use std::rc::Rc;

fn main() {
    let mut y = Box::new(5);
    println!("{:p}", y);
    if *y == 5 {
        println!("{:p} {}", y, *y);
    }
    *y = 10;
    println!("{:p} {}", y, *y);

    let x = Rc::new(5);
    if *x == 5 {
        println!("{:p} {}", x, *x);
    }
    // *x = 10;
}
