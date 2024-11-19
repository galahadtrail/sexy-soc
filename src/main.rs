mod capture;
mod output;
mod rules;

use capture::traffic_interception;
use output::print_hello_message;
use rules::{read_from_file, write_to_file, Rule};

mod prelude {
    pub use pnet::datalink::NetworkInterface;
    pub use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
    pub use pnet::packet::ipv4::Ipv4Packet;
    pub use pnet::packet::Packet;
    pub use std::fs::File;
    pub use std::io::{BufReader, BufWriter, Write};
    pub use std::net::IpAddr;
    pub use std::net::Ipv4Addr;
}

fn main() {
    print_hello_message();
    //traffic_interception();
}
