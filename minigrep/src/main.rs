use minigrep::Config;
use std::env;// args.collect
use std::process;

fn main() {
    // 1. 获取命令行参数
    // cargo run xxx xxx.txt
    let args: Vec<String> = env::args().collect(); // ["target\\debug\\minigrep.exe", "1234", "fafd.txt"]
    let config: Config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments:{}",err);
        process::exit(1);
    });

    // 2. 读取文件
    if let Err(e) = minigrep::run(config){
        print!("Application error:{}",e);
        process::exit(1);
    }
}


