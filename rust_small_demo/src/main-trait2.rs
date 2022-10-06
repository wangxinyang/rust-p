trait GetName {
    fn get_name(&self) -> &str;
}

trait PrintName {
    fn print_name(&self);
}

impl<T: GetName> PrintName for T {
    fn print_name(&self) {
        println!("name is :{}", self.get_name());
    }
}

struct Student {
    name: String,
}

impl GetName for Student {
    fn get_name(&self) -> &str {
        &self.name[0..]
    }
}

fn main() {
    let student = Student {
        name: String::from("Tosei"),
    };
    student.print_name();
    println!("{}", student.get_name());
}
