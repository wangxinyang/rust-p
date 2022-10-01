use clap::{
    builder::{BoolishValueParser, TypedValueParser},
    Arg, Command,
};

fn main() {
    let cmd = Command::new("mycmd").arg(
        Arg::new("flag")
            .long("flag")
            .action(clap::ArgAction::SetTrue)
            .value_parser(BoolishValueParser::new().map(|b| -> usize {
                if b {
                    10
                } else {
                    5
                }
            })),
    );

    let matches = cmd.clone().try_get_matches_from(["mycmd"]).unwrap();

    println!("{:?}", matches.get_one::<usize>("flag").copied());

    let matches = cmd.try_get_matches_from(["mycmd"]).unwrap();
    println!("{:?}", matches.get_one::<usize>("flag").copied());
}
