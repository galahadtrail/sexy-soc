use crate::prelude::*;

pub fn handle_ipv4_packet(interface_name: &str, ethernet: &EthernetPacket) {
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

pub fn handle_ethernet_frame(interface: &NetworkInterface, ethernet: &EthernetPacket) {
    let interface_name = &interface.name[..];
    match ethernet.get_ethertype() {
        EtherTypes::Ipv4 => handle_ipv4_packet(interface_name, ethernet),
        _ => (),
    }
}
