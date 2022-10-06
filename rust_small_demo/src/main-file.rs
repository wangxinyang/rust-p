use std::{
    fs::File,
    io::{Error, Read, Write},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
enum Gender {
    Unspecified = 0,
    Male = 1,
    Female = 2,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u8,
    gender: Gender,
}

impl User {
    fn new(name: String, age: u8, gender: Gender) -> Self {
        Self { name, age, gender }
    }

    fn persist(&self, filename: &str) -> Result<usize, Error> {
        let mut file = File::create(filename)?;
        let data = serde_json::to_string(&self)?;
        println!("{}", data);
        file.write_all(data.as_bytes())?;
        Ok(data.len())
    }

    fn load(&self, filename: &str) -> Result<Self, Error> {
        let mut file = File::open(filename)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let user: User = serde_json::from_str(&data)?;
        Ok(user)
    }
}

fn main() {
    let user = User::new("tosei".to_string(), 18, Gender::Male);
    let result = user.persist("test.txt").unwrap();
    println!("{:?}", result);
    let content = user.load("test.txt").unwrap();
    println!("content is: {:?}", content);
}
