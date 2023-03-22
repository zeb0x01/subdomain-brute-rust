#![allow(unused)]
#![allow(special_module_name)]

use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod lib;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let (target, file_path) = parse_config(&args);
        print_lines_in_groups_of_five(target, file_path);
        // read_file(target, file_path);
    }
    else {
        print!("Usage: ./target/debug/subdomain-brute-rust <target> <file_path> <output_file>");
    }
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];
    (query, file_path)
}

// fn read_file(target: &str, file_path: &str) {
//     let content = fs::read_to_string(file_path).expect("Exceptions");
//     for line in content.lines() {
//         let new_target = format!("{}.{}", line, target);
//         // use await 
//         lib::dns_queries(&new_target);
//     }
// }

fn print_lines_in_groups_of_five(target: &str, filename: &str) {
    if let Ok(file) = File::open(filename) {
        let reader = BufReader::new(file);

        let mut lines: Vec<String> = Vec::new();
        for line in reader.lines() {
            if let Ok(line) = line {
                lines.push(line);
                if lines.len() == 5000 {
                    for line in &lines {
                        let new_target = format!("{}.{}", line, target);
                        lib::dns_queries(&new_target);
                    }
                }
            }
        }
        if !lines.is_empty() {
            for line in &lines {
                let new_target = format!("{}.{}", line, target);
                lib::dns_queries(&new_target);
            }
        }
    } else {
        eprintln!("Error: could not open file {}", filename);
    }
}