use std::rc::Rc;

#[derive(Debug)]
struct Transfer {
    trans: Rc<Vec<u8>>,
    message: String,
}

impl Transfer {
    fn new(trans: &[u8], message: &str) -> Self {
        Transfer {
            trans: Rc::new(trans.to_owned()),
            message: message.to_owned(),
        }
    }

    fn trans(&self) -> Vec<u8> {
        self.trans.clone().to_vec()
    }

    fn message(&self) -> String {
        self.message.clone()
    }
}

fn main() {
    let transfer = Transfer::new(b"tosei", "hello");
    let trans = transfer.trans();
    let message = transfer.message();
    // Vec -> [u8]
    let trans_array: [u8; 5] = trans.try_into().unwrap();
    println!("trans_array is: {:?}", trans_array);

    // String  -> [u8]
    let message_array: [u8; 5] = message.as_bytes().try_into().unwrap();
    println!("message_array is: {:?}", message_array);

    // [u8] -> &str
    let trans_str = std::str::from_utf8(&trans_array).unwrap();
    println!("trans_str is: {:?}", trans_str);
}
