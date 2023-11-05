use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        // 判断是否设置环境变量
        let case_insensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_insensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 错误处理使用config一样的处理方式 ?处理方式是把错误返回给调用者
    let contents = fs::read_to_string(config.filename)?;
    // 打印结果
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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // let query = query.to_lowercase();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }
    // results
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe,fast,productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe,fast,productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "dUct";
        let contents = "\
Rust:
safe,fast,productive.
Pick three.
Duct tape.";
        assert_eq!(
            vec!["safe,fast,productive.", "Duct tape."],
            search_case_insensitive(query, contents)
        );
    }
}
