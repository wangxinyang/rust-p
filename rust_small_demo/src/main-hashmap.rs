use std::collections::HashMap;

fn main() {
    // let teams = vec![String::from("Blue"), String::from("Green")];

    // let inital_scores = vec![10, 50];

    // let scores: HashMap<_, _> = teams.iter().zip(inital_scores.iter()).collect();

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name);

    // match score {
    //     Some(score) => println!("{}", score),
    //     None => println!("team not exist"),
    // }
    for (k, v) in &scores {
        println!("{}-{}", k, v)
    }
}
