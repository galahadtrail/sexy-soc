use core::fmt;

use crate::prelude::*;

#[derive(serde_derive::Deserialize, serde_derive::Serialize, Debug, PartialEq, Eq, Clone)]
pub(crate) struct Rule {
    pub source: Ipv4Addr,
    pub destination: Ipv4Addr,
}

impl Rule {
    pub fn new(sourc: String, dest: String) -> Rule {
        Rule {
            source: sourc.parse().expect("Wrong source!"),
            destination: dest.parse().expect("Wrong destination!"),
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

pub fn add_rule(rules: &mut Vec<Rule>, new_rule: Rule) {
    rules.push(new_rule);
}

pub fn del_rule(rules: &mut Vec<Rule>, deprecated_rule: Rule) {
    rules.retain(|elem| *elem != deprecated_rule);
}

pub fn add_rule_from_console(rules: &mut Vec<Rule>) {
    println!("Write please source and destination IP's\nSource:");
    let mut source = String::new();
    let _ = io::stdin().read_line(&mut source);

    println!("Destination:");
    let mut destination = String::new();
    let _ = io::stdin().read_line(&mut destination);

    let new_rule = Rule::new(source, destination);
}
