use crate::prelude::*;
extern crate pnet;

use crate::rules::Rule;
use pnet::datalink;

use core::fmt;
use std::env;
use std::io::{self, Write};
use std::process;

#[derive(Debug)]
pub struct Alert {
    ttl: u8,
    flags: u8,
    version: u8,
    source: Ipv4Addr,
    destination: Ipv4Addr,
}

impl fmt::Display for Alert {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Achtung! (source: {}, destination: {}, TTL: {}, Flags: {}, Version: {})",
            self.source, self.destination, self.ttl, self.flags, self.version
        )
    }
}

pub fn print_all_net_alert(alerts: &mut Vec<Alert>) {
    for alert in alerts.iter() {
        println!("{}", alert);
    }
}

fn gimme_alert_if_it_here<'a>(some_packet_rule: &'a Rule, rules: &'a mut Vec<Rule>) -> bool {
    for rule in rules {
        if rule == some_packet_rule {
            return true;
        }
    }
    false
}

fn handle_ipv4_packet(ethernet: &EthernetPacket, rules: &mut Vec<Rule>, alerts: &mut Vec<Alert>) {
    let header = Ipv4Packet::new(ethernet.payload());
    if let Some(header) = header {
        let new_rule = Rule {
            source: header.get_source(),
            destination: header.get_destination(),
        };
        let mut _alert = match gimme_alert_if_it_here(&new_rule, rules) {
            true => {
                let new_alert = Alert {
                    ttl: header.get_ttl(),
                    flags: header.get_flags(),
                    version: header.get_version(),
                    source: header.get_source(),
                    destination: header.get_destination(),
                };
                let _ = write_current_dt_to_log(
                    "logs/net_alerts.log",
                    "success",
                    &format!("Achtung! {:?}", new_alert),
                );
                println!("{}", new_alert);
                alerts.push(new_alert);
            }
            false => (),
        };
    }
}

fn handle_ethernet_frame(
    ethernet: &EthernetPacket,
    rules: &mut Vec<Rule>,
    alerts: &mut Vec<Alert>,
) {
    match ethernet.get_ethertype() {
        EtherTypes::Ipv4 => handle_ipv4_packet(ethernet, rules, alerts),
        _ => (),
    }
}

pub fn traffic_interception(rules: &mut Vec<Rule>, alerts: &mut Vec<Alert>) {
    use pnet::datalink::Channel::Ethernet;
    let iface_name = match env::args().nth(1) {
        Some(n) => n,
        None => {
            writeln!(io::stderr(), "USAGE: packetdump <NETWORK INTERFACE>").unwrap();
            process::exit(1);
        }
    };
    let interface_names_match = |iface: &NetworkInterface| iface.name == iface_name;

    // Find the network interface with the provided name
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .filter(interface_names_match)
        .next()
        .unwrap_or_else(|| panic!("No such network interface: {}", iface_name));

    // Create a channel to receive on
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("packetdump: unhandled channel type"),
        Err(e) => panic!("packetdump: unable to create channel: {}", e),
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                handle_ethernet_frame(&EthernetPacket::new(packet).unwrap(), rules, alerts);
            }
            Err(_e) => return,
        }
    }
}
