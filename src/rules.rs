use crate::prelude::*;

#[derive(serde_derive::Deserialize, serde_derive::Serialize)]
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
