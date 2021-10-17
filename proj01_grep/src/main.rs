use std::env;
use proj01_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    let config = Config::new(&args);

    println!("archivo: {}", config.filename);
    println!("string a buscar: {}", config.query);

    proj01_grep::run(config);
}


