use crate::prelude::*;
use crate::rules::rules_endpoint;
use std::process;

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Menu,
    TrafficInterception,
    ComputerInformation,
    NetworkRulesChanging,
    ComputerRulesChanging,
    Exit,
}

pub fn infinite_action_loop(privilege: &Privileges, rules: &mut Vec<Rule>) {
    loop {
        let mut welcome = Statement::Menu;
        welcome = menu(welcome);

        match welcome {
            Statement::Menu => continue,
            Statement::TrafficInterception => println!("1"),
            Statement::ComputerInformation => println!("2"),
            Statement::NetworkRulesChanging => rules_endpoint(rules, privilege),
            Statement::ComputerRulesChanging => println!("4"),
            Statement::Exit => {
                let _ = write_to_file(rules.to_vec());
                process::exit(0)
            }
        }
    }
}

fn menu(mut welcome: Statement) -> Statement {
    println!("Please tell me what you want to do\n\n0. Menu again\n1. Traffic Interception\n2. Computer Information\n3. Network Rules Changing\n4. Computer Rules Changing\n5. Exit");
    let mut option = String::new();
    let _ = io::stdin().read_line(&mut option);

    welcome = match option.as_str().trim() {
        "0" => Statement::Menu,
        "1" => Statement::TrafficInterception,
        "2" => Statement::ComputerInformation,
        "3" => Statement::NetworkRulesChanging,
        "4" => Statement::ComputerRulesChanging,
        "5" => Statement::Exit,
        _ => Statement::Exit,
    };
    welcome
}
