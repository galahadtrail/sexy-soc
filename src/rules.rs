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

pub fn write_hash_rules_from_file(path_to_file: &str, rules: Vec<String>) -> std::io::Result<()> {
    let rules_jsoned: Vec<String> = rules
        .iter()
        .map(|rule| serde_json::to_string(rule).unwrap())
        .collect();

    let file = File::create(path_to_file)?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &rules_jsoned)?;
    writer.flush()?;

    Ok(())
}

pub fn read_hash_rules_from_file(path_to_file: &str) -> io::Result<Vec<String>> {
    let file = File::open(path_to_file)?;
    let reader = BufReader::new(file);
    let rules_raw: Vec<String> = serde_json::from_reader(reader)?;

    let rules_unjsoned: Vec<String> = rules_raw
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

fn add_hash_rule(hash_rules: &mut Vec<String>) {
    let mut temp_for_output = "Write please your new hash:"
        .truecolor(193, 251, 222)
        .on_purple();

    println!("{}", temp_for_output);

    let mut new_hash = String::new();
    let _ = io::stdin().read_line(&mut new_hash);
    hash_rules.push(new_hash);
}

fn del_hash_rule(hash_rules: &mut Vec<String>) {
    let mut temp_for_output = "Write please your deprecated hash:"
        .truecolor(193, 251, 222)
        .on_purple();

    println!("{}", temp_for_output);
    let mut depr_hash_rule = String::new();
    let _ = io::stdin().read_line(&mut depr_hash_rule);
    hash_rules.retain(|elem| *elem != depr_hash_rule);
}

pub fn hash_rules_endpoint(hash_rules: &mut Vec<String>, privilege: &Privileges) {
    if *privilege != Privileges::Admin {
        return;
    }

    let temp_for_output = "Here are all rules you have:"
        .truecolor(193, 251, 222)
        .on_purple();

    println!("{}", temp_for_output);

    for rule in hash_rules.iter() {
        println!("{}", rule);
    }

    println!("Please tell me what you want to do.\n1. Add rule\n2. Delete rule\n3. Exit");

    let mut option = String::new();
    let _ = io::stdin().read_line(&mut option);

    match option.as_str().trim() {
        "1" => add_hash_rule(hash_rules),
        "2" => del_hash_rule(hash_rules),
        "3" => return,
        _ => hash_rules_endpoint(hash_rules, privilege),
    };
}
