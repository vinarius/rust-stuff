use std::env::args;

struct Config {

}

fn main() {
    let args: Vec<String> = args().collect();
    let name = &args[1];
    println!("Hello, {}!", name);
}
