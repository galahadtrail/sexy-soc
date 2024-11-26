use crate::prelude::*;
extern crate pnet;

use crate::rules::Rule;
use pnet::datalink;

use std::env;
use std::io::{self, Write};
use std::process;

fn gimme_alert_if_it_here<'a>(
    some_packet_rule: &'a Rule,
    rules: &'a mut Vec<Rule>,
    header: &'a Ipv4Packet<'a>,
) -> Option<&'a Ipv4Packet<'a>> {
    for rule in rules {
        if rule == some_packet_rule {
            return Some(header);
        }
    }
    None
}

fn handle_ipv4_packet(interface_name: &str, ethernet: &EthernetPacket) {
    let header = Ipv4Packet::new(ethernet.payload());
    if let Some(header) = header {
        println!(
            "IPv4 Packet: interface: {}, source: {}, destination: {}, payload: {:?}",
            interface_name,
            IpAddr::V4(header.get_source()),
            IpAddr::V4(header.get_destination()),
            header.payload()
        )
    } else {
        println!("[{}]: Malformed IPv4 Packet", interface_name);
    }
}

fn handle_ethernet_frame(interface: &NetworkInterface, ethernet: &EthernetPacket) {
    let interface_name = &interface.name[..];
    match ethernet.get_ethertype() {
        EtherTypes::Ipv4 => handle_ipv4_packet(interface_name, ethernet),
        _ => (),
    }
}

pub fn traffic_interception() {
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
                handle_ethernet_frame(&interface, &EthernetPacket::new(packet).unwrap());
            }
            Err(e) => panic!("packetdump: unable to receive packet: {}", e),
        }
    }
}
