use pcap::Device;

fn main() {
    let mut cap = Device::lookup().unwrap().unwrap().open().unwrap();

    while let Ok(packet) = cap.next_packet() {
        println!("received packet! {:?}", packet);
    }
}
