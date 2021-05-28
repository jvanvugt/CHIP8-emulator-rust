use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Expected exactly 2 arguments");
        return
    }
    let rom_file = &args[1];
    let contents = std::fs::read(rom_file).unwrap();
}
