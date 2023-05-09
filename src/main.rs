use base64::{engine::general_purpose, Engine as _};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} <encode|decode> <input>", args[0]);
        std::process::exit(1);
    }

    let command = &args[1];
    let input = &args[2..].join(" ");

    if command == "encode" {
        let encoded = general_purpose::STANDARD.encode(input);
        println!("{}", encoded);
    } else if command == "decode" {
        let decoded = general_purpose::STANDARD.decode(input).unwrap();
        let output = String::from_utf8(decoded).unwrap();
        println!("{}", output);
    } else {
        println!("Unknown command: {}", command);
        std::process::exit(1);
    }
}
