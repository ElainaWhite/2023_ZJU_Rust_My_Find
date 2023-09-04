use std::env;
use std::process;
use regex::Regex;
use colored::Colorize;

mod interpreter;
mod find;

fn main() {
    let args : Vec<String> = env::args().collect();

    let mut paths : Vec<String> = Vec::new();
    let mut regexs : Vec<Regex> = Vec::new();

    interpreter::interpreter(&args,&mut paths,&mut regexs);
    for path in paths{
        match find::find(&path,&regexs) {
            Ok(matches) => {
                if matches.is_empty() {
                    println!("{}",path.blue());
                    println!("\tNo file expected found!");
                }else {
                    for file in matches{
                        println!("\t{}",file.green());
                    }
                }
            }
            Err(error) => {
                eprintln!("Error: {}",error);
                process::exit(1);
            }
        }
    }

}




