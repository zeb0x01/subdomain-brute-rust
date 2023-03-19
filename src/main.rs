#![allow(unused)]
#![allow(special_module_name)]

use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::vec;

mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let (target, file_path) = parse_config(&args);
        read_file(target, file_path);
    } else {
        print!("Usage: ./target/debug/subdomain-brute-rust <target> <file_path>");
    }
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];
    (query, file_path)
}

fn read_file(target: &str, file_path: &str) {
    let mut content = fs::read_to_string(file_path).expect("Exceptions");
    for line in content.lines() {
        let new_target = format!("{}.{}", line, target);
        lib::dns_queries(new_target.as_str());
    }
}
