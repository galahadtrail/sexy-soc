mod authorization;
mod capture;
mod menu;
mod output;
mod rules;

use authorization::authorize;
use capture::traffic_interception;
use menu::Statement;
use output::print_hello_message;
use prelude::*;
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print_hello_message();

    let privileges = authorize()?;

    let mut welcome = Statement::Menu;

    loop {
        println!("Please tell me what you want to do\n\n1. Traffic Interception\n2. Computer Information\n3. Network Rules Changing\n4. Computer Rules Changing");
        let mut option = String::new();
        let _ = io::stdin().read_line(&mut option);

        welcome = match option.as_str() {
            "1" => Statement::TrafficInterception,
            "2" => Statement::ComputerInformation,
            "3" => Statement::NetworkRulesChanging,
            "4" => Statement::ComputerRulesChanging,
            _ => Statement::Menu,
        };

        match welcome {
            Statement::Menu => continue,
            Statement::TrafficInterception => println!("1"),
            Statement::ComputerInformation => println!("2"),
            Statement::NetworkRulesChanging => println!("3"),
            Statement::ComputerRulesChanging => println!("4"),
        }
    }

    //traffic_interception();
    Ok(())
}
