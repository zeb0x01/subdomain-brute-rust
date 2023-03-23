use std::env;
use std::fs;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::net::IpAddr;
use futures::future::join_all;
use tokio::task;
use dns_lookup::lookup_host;

mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let (target, file_path) = parse_config(&args);
        read_file(target, file_path);
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
    let pool = threadpool::Builder::new().num_threads(4).build();
    let results: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    for line in lines {
        let target = target.to_string();
        let line = line.to_string();
        let results = Arc::clone(&results);
        pool.execute(move || {
            let target = line + "." + &target;
            let result = dns_queries(&target);
            if result {
                results.lock().unwrap().push(target);
            }
        });
    }
    pool.join();
    let results = results.lock().unwrap();
    for result in results.iter() {
        println!("{}", result);
    }
}

async fn dns_query(target: &str) -> Vec<IpAddr> {
    let ips = lookup_host(target).unwrap_or_else(|_| Vec::new());
    ips
}

fn dns_queries(target: &str) -> bool {
    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    let ips = rt.block_on(dns_query(target));
    if !ips.is_empty() {
        for ip in ips {
            println!("{}: {}", target, ip);
        }
        true
    } else {
        false
    }
}
