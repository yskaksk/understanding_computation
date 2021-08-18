use std::env;

use regex::parse::parse;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 3 {
        eprintln!("too much args");
        std::process::exit(1);
    }
    let regex = &args[1];
    let pat = parse(regex.to_string());
    println!("{}", pat.matches(String::from(&args[2])));
}
