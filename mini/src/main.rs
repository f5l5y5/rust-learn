use mini::Config;
use std::env;
use std::process;

fn main() {
    //本来就是迭代器
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments:{}", err);
        process::exit(1);
    });

    if let Err(e) = mini::run(config) {
        eprintln!("Application error:{}", e);
        process::exit(1);
    }
}
