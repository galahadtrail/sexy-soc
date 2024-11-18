use crate::prelude::*;

#[derive(serde_derive::Deserialize, serde_derive::Serialize)]
struct Rule {
    source: Ipv4Addr,
    destination: Ipv4Addr,
}

impl Rule {
    pub fn new(sourc: Ipv4Addr, dest: Ipv4Addr) -> Rule {
        Rule {
            source: sourc,
            destination: dest,
        }
    }
    pub fn read_from_file(rules: Vec<Rule>) {}
}
