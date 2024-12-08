mod authorization;
mod capture;
mod connection;
mod menu;
mod output;
mod rules;

use authorization::authorize;
use menu::infinite_action_loop;
use output::print_hello_message;

mod prelude {
    pub use crate::authorization::Privileges;
    pub use crate::capture::Alert;
    pub use crate::connection::ComputerAlert;
    pub use crate::rules::{
        read_from_file, read_hash_rules_from_file, write_hash_rules_from_file, write_to_file, Rule,
    };
    pub use colored::*;
    pub use pnet::datalink::NetworkInterface;
    pub use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
    pub use pnet::packet::ipv4::Ipv4Packet;
    pub use pnet::packet::Packet;
    pub use std::fs::File;
    pub use std::io;
    pub use std::io::{BufRead, BufReader, BufWriter, Error, Write};
    pub use std::net::Ipv4Addr;
    pub use std::sync::{Arc, Mutex};
    pub use std::thread;
}

use prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print_hello_message();

    let privileges = authorize()?;

    ctrlc::set_handler(move || {
        println!("Получен сигнал Ctrl+C! Выход из функции перехвата");
        return;
    })
    .expect("Ошибка при установке обработчика Ctrl+C");

    let mut rules = read_from_file().expect("Something wrong with reading rules from file");
    let mut alerts: Vec<Alert> = Vec::new();
    let mut hash_rules = read_hash_rules_from_file("src/rules/rules.txt")
        .expect("Something wrong with reading hash rules from file");
    let hash_alerts: Arc<Mutex<Vec<ComputerAlert>>> = Arc::new(Mutex::new(Vec::new()));

    infinite_action_loop(
        &privileges,
        &mut rules,
        &mut alerts,
        &mut hash_rules,
        hash_alerts,
    );

    Ok(())
}
