use std::cell::RefCell;

#[derive(Debug)]
struct Employee {
    name: String,
}

impl Employee {
    fn change_name(&mut self, name: String) {
        self.name = name
    }
}

fn main() {
    let employee = RefCell::new(Employee {
        name: String::from("张三"),
    });

    {
        let mut r = employee.borrow_mut();
        r.change_name(String::from("王五"));
    }

    println!("{:?}", employee);
    println!("{:?}", employee.borrow());
}
