mod capture;
mod output;
mod rules;

use capture::traffic_interception;
use output::print_hello_message;
use rules::{write_to_file, Rule};

mod prelude {
    pub use pnet::datalink::NetworkInterface;
    pub use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
    pub use pnet::packet::ipv4::Ipv4Packet;
    pub use pnet::packet::Packet;
    pub use std::fs::File;
    pub use std::io::{BufWriter, Write};
    pub use std::net::IpAddr;
    pub use std::net::Ipv4Addr;
}

fn main() {
    print_hello_message();
    //traffic_interception();
    let rls = Rule::new("127.0.0.1".parse().unwrap(), "127.0.0.1".parse().unwrap());
    let rls1 = Rule::new(
        "192.168.70.1".parse().unwrap(),
        "189.23.134.1".parse().unwrap(),
    );
    let vec = vec![rls, rls1];
    let _ = write_to_file(vec);
}
