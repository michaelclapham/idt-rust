use std::env;

fn main() {
    // Use arguments to decide command and which file to apply to
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() < 3 {
        println!("Usage: idt-rust [COMMAND] [FILENAME]");
        println!("Example: idt-rust extract test.idml");
        std::process::exit(-1);
    }

    match args[1].as_str() {
        "extract" => extract(args[2].as_str()),
        "translate" => translate(args[2].as_str()),
        _ => println!("I don't know the command {:?}", args[1])
    }
}

fn extract(filename: &str) {
    println!("Extracting strings from {:?}", filename);
}

fn translate(filename: &str) {
    println!("Translating file {:?}", filename);
}