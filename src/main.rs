mod authorization;
mod capture;
mod menu;
mod output;
mod rules;

use authorization::authorize;
use menu::infinite_action_loop;
use output::print_hello_message;
use rules::read_from_file;

mod prelude {
    pub use crate::rules::{read_from_file, Rule};
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

    let mut rules = read_from_file().expect("Something wrong with reading rules from file");

    let clone_prevs = privileges.clone();

    let mut clone_rules = rules.clone();

    ctrlc::set_handler(move || {
        println!("\nПолучен сигнал Ctrl+C! Возврат к меню.");
        infinite_action_loop(&privileges, &mut rules);
    })
    .expect("Ошибка при установке обработчика Ctrl+C");

    infinite_action_loop(&clone_prevs, &mut clone_rules);

    Ok(())
}
