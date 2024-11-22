mod authorization;
mod capture;
mod menu;
mod output;
mod rules;

use authorization::authorize;
use capture::traffic_interception;
use menu::{
    ComputerRulesChanging, ComputersInformation, Menu, NetworkRulesChanging, TrafficInterception,
};
use output::print_hello_message;
use rules::{read_from_file, write_to_file, Rule};

mod prelude {
    pub use colored::*;
    pub use pnet::datalink::NetworkInterface;
    pub use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
    pub use pnet::packet::ipv4::Ipv4Packet;
    pub use pnet::packet::Packet;
    pub use std::fs::File;
    pub use std::io;
    pub use std::io::{BufRead, BufReader, BufWriter, Error, Write};
    pub use std::net::IpAddr;
    pub use std::net::Ipv4Addr;
}

fn main() {
    print_hello_message();
    let privileges = authorize();
    println!("{:?}", privileges);
    let mut goodmorning = Menu::new();
    goodmorning.to_computer_rules_changing();
    let res = goodmorning.to_network_rules_changing();
    println!("{:?}", res);
    //traffic_interception();
}
