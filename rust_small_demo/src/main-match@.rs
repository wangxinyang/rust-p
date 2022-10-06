fn main() {
    // let _x = 5;
    // let _y = 6;

    // let s = Some(String::from("hello"));
    // if let Some(_) = s {
    //     println!("found a string");
    // }
    // println!("s:{:?}", s);
    // println!("hello world");
    let msg = Message::Hello(5);

    match msg {
        Message::Hello(id_val @ 3..=7) => {
            println!("id_val = {}", id_val);
        }
        Message::Hello(id_val @ 10..=20) => {
            println!("large");
        }
        Message::Hello(id_val) => {
            println!("id_val = {}", id_val);
        }
    }
}

enum Message {
    Hello(i32),
}
