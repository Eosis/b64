use base64;

enum Operation {
    Decode,
    Encode
}

const USAGE: &str = "b64 <-d|-e> <input>";

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        println!("{}", USAGE);
    }

    let [op, input] = [&args[0], &args[1]];

    let op = match op.as_str() {
        "-d" => Operation::Decode,
        "-e" => Operation::Encode,
        _ => panic!(USAGE)
    };

    match op {
        Operation::Decode => println!("{}", run_decode(&input)),
        Operation::Encode => println!("{}", run_encode(&input)),
    }
}

fn run_decode(input: &str) -> String {
    let bytes = base64::decode(input)
        .expect("Failed to decode the base64 string. Are you sure it is a valid base64 encoding?");
    String::from_utf8(bytes)
        .expect("Couldn't convert the bytes to a valid str. Was this just a utf8 string that was encoded?")
}

fn run_encode(input: &str) -> String {
    base64::encode(input.as_bytes())
}