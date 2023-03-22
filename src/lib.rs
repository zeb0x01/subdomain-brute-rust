#![allow(unused)]

extern crate dns_lookup;
use dns_lookup::lookup_host;
pub fn dns_queries(target: &str) -> bool {
    let ips = lookup_host(target);
    match ips {
        Ok(ips) => {
            for ip in ips {
                println!("{}: {}", target, ip);
            }
        }
        Err(e) => {
            // println!("{}: {}", target, e);
            return false;
        }
    }
    return true;
}
