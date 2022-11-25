use std::{fs::File, io::BufReader};

use rodio::Sink;
fn main() {
    // _stream不能写成_, 会报
    // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: NoDevice', practice_coding/patience/src/main.rs:6:40
    let (_stream, handler) = rodio::OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handler).unwrap();

    let file = File::open("assets/music.mp3").unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

    sink.sleep_until_end();
}
