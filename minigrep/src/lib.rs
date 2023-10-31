use std::error::Error;
use std::fs; // read_to_string
use std::env; // args.collect

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 错误处理使用config一样的处理方式 ?处理方式是把错误返回给调用者
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_insensitive {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_insensitive = env::var("CASE_INSENSITIVE").is_err(); // 检查是否设置了环境变量
        Ok(Config { query, filename,case_insensitive })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // vec![]
    // 检查每一行是否包含query 如果包含就将其加入到结果中
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // 检查每一行是否包含query 如果包含就将其加入到结果中
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_insensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe,fast,productive.
Duct three.";
        assert_eq!(vec!["safe,fast,productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "rUSt";
        let contents = "\
Rust:
safe,fast,productive.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
