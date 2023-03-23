#![allow(special_module_name)]

use std::env;
use std::fs;
use std::thread;
use std::time::Duration;

mod lib;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let (target, file_path) = parse_config(&args);
        read_file(target, file_path);
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


fn read_file(target: &str, filename: &str) {
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = content.lines().collect();
    let mut threads = vec![];
    for line in lines {
        let target = target.to_string();
        let line = line.to_string();
        let handle = thread::spawn(move || {
            let target = line + "." + &target;
            let result = lib::dns_queries(&target);
            if result {
                println!("{}", target);
            }
        });
        threads.push(handle);
        thread::sleep(Duration::from_millis(1));
    }

}
