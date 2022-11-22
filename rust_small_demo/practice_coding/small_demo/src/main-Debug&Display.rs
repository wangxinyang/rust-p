use std::fmt::Display;

fn main() {
    println!("{:?}", Structure(3));

    println!("Now {} will print!", Deep(Structure(7)));
}

#[derive(Debug)]
struct Structure(i32);

// #[derive(Debug)]
struct Deep(Structure);

// impl Display for Structure {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }

impl Display for Deep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0 .0)
    }
}
