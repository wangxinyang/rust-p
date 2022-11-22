#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state Quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    // value_in_cents(Coin::Penny);
    // value_in_cents(Coin::Quarter(UsState::Alabama));

    let v = Some(3);

    match v {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = v {
        println!("hahah")
    }
}
