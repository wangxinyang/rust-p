use std::{fmt::Display, fs::File, io::Write};

struct User {
    name: String,
    age: usize,
    gender: Gender,
}

#[allow(dead_code)]
enum Gender {
    Unsure = 0,
    Male = 1,
    Female = 2,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name is: {}, age is: {}, gender is: {}",
            self.name, self.age, self.gender
        )
    }
}

impl Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gender::Unsure => write!(f, "不确定"),
            Gender::Male => write!(f, "男性"),
            Gender::Female => write!(f, "女性"),
        }
    }
}

fn main() {
    let user = User {
        name: "tosei".to_string(),
        age: 40,
        gender: Gender::Male,
    };
    let mut create_file = File::create("user.txt").unwrap();
    create_file.write_all(user.to_string().as_bytes()).unwrap();
}
