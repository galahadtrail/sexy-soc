mod capture;
mod output;
mod rules;

use capture::traffic_interception;
use output::print_hello_message;

mod prelude {
    pub use pnet::datalink::NetworkInterface;
    pub use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
    pub use pnet::packet::ipv4::Ipv4Packet;
    pub use pnet::packet::Packet;
    pub use std::net::IpAddr;
}

fn main() {
    print_hello_message();
    traffic_interception();
}
