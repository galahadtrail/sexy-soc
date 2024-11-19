use core::fmt;

use crate::prelude::*;

#[derive(serde_derive::Deserialize, serde_derive::Serialize, Debug, PartialEq, Eq)]
pub(crate) struct Rule {
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

    pub fn match_with(&self, another: &Rule) -> bool {
        if self == another {
            return true;
        }
        false
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(source: {}, destination: {})",
            self.source, self.destination
        )
    }
}

pub fn write_to_file(rules: Vec<Rule>) -> std::io::Result<()> {
    let rules_jsoned: Vec<String> = rules
        .iter()
        .map(|rule| serde_json::to_string(rule).unwrap())
        .collect();

    let file = File::create("src/rules/rules_ipv4.txt")?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &rules_jsoned)?;
    writer.flush()?;

    Ok(())
}

pub fn read_from_file() -> std::io::Result<Vec<Rule>> {
    let file = File::open("src/rules/rules_ipv4.txt")?;
    let reader = BufReader::new(file);
    let rules_raw: Vec<String> = serde_json::from_reader(reader)?;

    let rules_unjsoned: Vec<Rule> = rules_raw
        .iter()
        .map(|rule| serde_json::from_str(&rule).unwrap())
        .collect();

    Ok(rules_unjsoned)
}
