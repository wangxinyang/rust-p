fn main() {
    if atty::is(atty::Stream::Stdout) {
        println!("i am a tty");
    } else {
        println!("i am not a tty");
    }
}
