use crate::capture::{print_all_net_alert, traffic_interception};
use crate::connection::{connection_start, print_all_hosts_alerts};
use crate::prelude::*;
use crate::rules::{hash_rules_endpoint, rules_endpoint};
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

pub fn infinite_action_loop(
    privilege: &Privileges,
    rules: &mut Vec<Rule>,
    alerts: &mut Vec<Alert>,
    hash_rules: &mut Vec<String>,
    hash_alerts: Arc<Mutex<Vec<ComputerAlert>>>,
) {
    loop {
        let welcome = menu();

        match welcome {
            Statement::Menu => continue,
            Statement::TrafficInterception => traffic_interception(rules, alerts),
            Statement::ComputerInformation => {
                let hash_rules = read_hash_rules_from_file("src/rules/rules.txt").unwrap();
                let rules_str = hash_rules.join("@");

                let should_run = Arc::new(Mutex::new(true));
                let should_run_clone = Arc::clone(&should_run);
                let hash_alert_clone = Arc::clone(&hash_alerts);

                thread::spawn(move || {
                    connection_start(should_run_clone, &rules_str, hash_alert_clone);
                });
                let mut input = String::new();

                let str = "Type 'exit' to turn off the connection.."
                    .truecolor(193, 251, 222)
                    .on_purple();
                println!("{}", str);
                loop {
                    input.clear();
                    io::stdin().read_line(&mut input).unwrap();
                    if input.trim() == "exit" {
                        *should_run.lock().unwrap() = false;
                        break;
                    }
                }
            }
            Statement::NetworkRulesChanging => rules_endpoint(rules, privilege),
            Statement::ComputerRulesChanging => hash_rules_endpoint(hash_rules, privilege),
            Statement::Exit => {
                let _ = write_to_file(rules.to_vec());
                let _ = write_hash_rules_from_file("src/rules/rules.txt", hash_rules.to_vec());

                let mut write = "All net alerts:".truecolor(193, 251, 222).on_purple();
                println!("{}", write);
                print_all_net_alert(alerts);

                write = "All agents alerts:".truecolor(193, 251, 222).on_purple();
                println!("{}", write);
                print_all_hosts_alerts(hash_alerts);

                process::exit(0)
            }
        }
    }
}

fn menu() -> Statement {
    let tell = "Please tell me what you want to do"
        .truecolor(193, 251, 222)
        .on_purple();
    println!("{}", tell);
    println!("\n0. Menu again\n1. Traffic Interception\n2. Computer Information\n3. Network Rules Changing\n4. Computer Rules Changing\n5. Exit");
    let mut option = String::new();
    let _ = io::stdin().read_line(&mut option);

    let welcome = match option.as_str().trim() {
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
