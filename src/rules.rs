use core::fmt;

use crate::prelude::*;

#[derive(serde_derive::Deserialize, serde_derive::Serialize, Debug, PartialEq, Eq, Clone)]
pub struct Rule {
    pub source: Ipv4Addr,
    pub destination: Ipv4Addr,
}

impl Rule {
    pub fn new(sourc: &str, dest: &str) -> Rule {
        Rule {
            source: sourc.parse::<Ipv4Addr>().expect("Wrong source!"),
            destination: dest.parse::<Ipv4Addr>().expect("Wrong destination!"),
        }
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

fn add_rule_from_console(rules: &mut Vec<Rule>) {
    let mut temp_for_output = "Write please source and destination IP's\nSource:"
        .truecolor(193, 251, 222)
        .on_purple();

    println!("{}", temp_for_output);
    let mut source = String::new();
    let _ = io::stdin().read_line(&mut source);

    temp_for_output = "Destination:".truecolor(193, 251, 222).on_purple();
    println!("{}", temp_for_output);
    let mut destination = String::new();
    let _ = io::stdin().read_line(&mut destination);

    let new_rule = Rule::new(source.trim(), destination.trim());
    rules.push(new_rule)
}

fn del_rule_from_console(rules: &mut Vec<Rule>) {
    let mut temp_for_output = "Write please source and destination IP's\nSource:"
        .truecolor(193, 251, 222)
        .on_purple();

    println!("{}", temp_for_output);
    let mut source = String::new();
    let _ = io::stdin().read_line(&mut source);

    temp_for_output = "Destination:".truecolor(193, 251, 222).on_purple();
    println!("{}", temp_for_output);
    let mut destination = String::new();
    let _ = io::stdin().read_line(&mut destination);

    let depr_rule = Rule::new(source.trim(), destination.trim());
    rules.retain(|elem| *elem != depr_rule);
}

pub fn rules_endpoint(rules: &mut Vec<Rule>, privilege: &Privileges) {
    if *privilege != Privileges::Admin {
        return;
    }

    let temp_for_output = "Here are all rules you have:"
        .truecolor(193, 251, 222)
        .on_purple();

    println!("{}", temp_for_output);

    for rule in rules.iter() {
        println!("{}", rule);
    }

    println!("Please tell me what you want to do.\n1. Add rule\n2. Delete rule\n3. Exit");

    let mut option = String::new();
    let _ = io::stdin().read_line(&mut option);

    match option.as_str().trim() {
        "1" => add_rule_from_console(rules),
        "2" => del_rule_from_console(rules),
        "3" => return,
        _ => rules_endpoint(rules, privilege),
    };
}
