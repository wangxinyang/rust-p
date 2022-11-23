mod pb;

use pb::*;
use prost::Message;

fn main() {
    let request = RequestGet {
        key: "key".to_string(),
    };
    let mut buf = Vec::new();
    request.encode(&mut buf).unwrap();
    println!("{:?}", buf);
}
