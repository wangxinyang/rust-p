trait GetInformation {
    fn get_name(&self) -> &str;
    fn get_age(&self) -> u32;
}

struct Student {
    name: String,
    age: u32,
}

struct Teacher {
    name: String,
    age: u32,
    subject: String,
}

impl Teacher {
    fn new(name: String, age: u32, subject: String) -> Self {
        Self { name, age, subject }
    }
}

impl Student {
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }
}

impl GetInformation for Student {
    fn get_name(&self) -> &str {
        &self.name[0..]
    }

    fn get_age(&self) -> u32 {
        self.age
    }
}

// fn print_information(item: impl GetInformation) {
//     println!("name = {}", item.get_name());
//     println!("age = {}", item.get_age());
// }

fn print_information<T: GetInformation>(item: T) {
    println!("name = {}", item.get_name());
    println!("age = {}", item.get_age());
}

fn main() {
    let student = Student::new(String::from("tosei"), 39);
    let teacher = Teacher::new(String::from("tosei"), 39, String::from("math"));
    // println!(
    //     "name is :{}, age is: {}",
    //     student.get_name(),
    //     student.get_age()
    // );
    // println!("{:?}", teacher.);
    print_information(student);
    // print_information(teacher);
}
