use regex::Regex;
use std::process;
pub fn interpreter(args: &Vec<String>, paths:&mut Vec<String>, regexs:&mut  Vec<Regex>) {
    let mut expressions : Vec<String> = Vec::new();
    if args.len() == 2 && args[1] == "-h" {
        print_help();
        process::exit(1);
    }

    if args.len() == 1 {
        // find
        paths.push(String::from("./"));
        expressions.push(String::from("-v"));
    } else if args.len() == 2 {
        // find {expression}
        paths.push(String::from("./"));
        expressions.push(args[1].clone());
    } else if args.len() == 3 {
        if args[1] == "-r" {
            // find -r {expression}
            paths.push(String::from("./"));
            expressions.push(args[2].clone());
        }else {
            // find {path} {expression}
            paths.push(args[1].clone());
            expressions.push(args[2].clone());
        }
    }else {
        if args[1] == "-r" {
            paths.push(String::from("./"));
            // find -r {expression 1} {expression 2} ...
            expressions = args[2..].to_vec();
        } else if args[1] != "-d" && args[2] == "-r" {
            // find {path} -r {expression 1} {expression 2} ...
        } else if args[1] == "-d" {
            if let Some(index) = args.iter().position(|s| s == "-r" ) {
                // find -d {path1} {path2} ... -r {expression 1} {expression 2} ...
                *paths = args[2..index].to_vec();
                expressions = args[index + 1..].to_vec();
            }else {
                // find -d {path1} {path2} ... {path n} {expression}
                let index = args.len() - 1;
                *paths = args[2..index].to_vec();
                expressions.push(args[index].clone())
            }

        }
    }
    
    if expressions.is_empty() || paths.is_empty() {
        println!("Error syntax: please use find -h for help!");
        process::exit(1);
    }
    for pattern in expressions{
        let new_regex = match  Regex::new(&pattern){
            Ok(re) =>re,
            Err(err) => {
                eprintln!("Invalid expression '{}':{}",pattern,err );
                process::exit(1);
            }
        };
        regexs.push(new_regex.clone());
    }
    paths.sort();
}

fn print_help() {
    println!("./find {{target directory }} {{expression }}: Find all the files and their paths in the target directory which match the expression;");
    println!("\tIf {{target directory}} is null, the target directory will be the current directory.");
    println!("\tIf both {{target directory}} and {{expression}} are null, it will list all the files of the current directory.");
    println!("\tIf the {{expression}} is -v or --verbose, it will list all the files of the target directory.");
    println!("./find -d {{target directory 1}} {{target directory 2}} ... -r {{expression 1}} {{expression 2}}  ...: Find all the files in every target directory satisfy any of the expressions.");
    println!("\t-d can't be null if you have more than one target directory, and it must be null if {{target directory}} is null.");
    println!("\t-r can't be null if you have more than one target directory.");
    println!("\tIf {{target directory}} is null, the target directory will be the current directory.");
    println!("{{expression}} can't be null except in ./find.");
}