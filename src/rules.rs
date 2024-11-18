use crate::prelude::*;

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
}
