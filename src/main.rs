use std::net::IpAddr;

use dns_lookup::lookup_host;

fn main() {
    let auth = vec![
        lookup_host("auth.factorio.com").unwrap(),
        (1..=4).map(|idx|
            lookup_host(&*format!("pingpong{}.factorio.com", idx)).unwrap()
        ).flatten().collect()
    ].iter().flatten()
        .map(|addr| format!("{}/{}", addr, match *addr {
            IpAddr::V4(_) => 32,
            IpAddr::V6(_) => 128,
        }))
        .collect::<Vec<String>>()
        .join(";");
    println!("allowed-ips=10.0.9.1/32;{};", auth);
}
