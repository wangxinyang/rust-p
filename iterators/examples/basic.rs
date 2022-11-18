fn main() {
    // let iter = vec!["a", "b", "c"].into_iter();
    let vs = vec!["s".to_string(), "t".to_string(), "r".to_string()];

    // equals for var in &vs
    for var in vs.iter() {
        // borrows vs, & to var
        println!("{}", var);
    }

    for var in vs {
        // consumes vs, owned var
        println!("{}", var);
    }
}
