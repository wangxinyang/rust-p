struct Dog {
    name: String,
}

impl Drop for Dog {
    fn drop(&mut self) {
        println!("the dog {} leave", self.name);
    }
}

fn main() {
    let wangcai = Dog {
        name: String::from("wangcai"),
    };
    println!("0----------------------");
    {
        let dahuang = Dog {
            name: String::from("dahuang"),
        };
    }
    println!("1----------------------");
}
