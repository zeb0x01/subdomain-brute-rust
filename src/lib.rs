#![allow(unused)]
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
            return false;
        }
    }
    return true;
}
