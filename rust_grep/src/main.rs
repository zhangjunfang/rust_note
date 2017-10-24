use std::error::Error;
use  std::env;
use std::fs::File;
use std::process;
use std::io::Read;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.is_empty() {
        println!("Hello, world!");
    } else {
        let config = Config::new(&args).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });
        if let Err(e) = run(config) {
            println!("Application error: {}", e);
            process::exit(1);
        }
    }

}
fn run(config: Config) -> Result<(), Box<Error>> {

    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;


   let r: Vec<&str> = search(&config.query, &contents );

    for line in &r {

        println!("{}",line);
    }

    Ok(())
}
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(query.to_lowercase()) {
            results.push(line);
        } else {

        }
    }

    results
}
struct Config {
    query: String,
    filename: String,
}
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}