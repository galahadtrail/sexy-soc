use crate::capture::{print_all_net_alert, traffic_interception};
use crate::connection::connection_start;
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

pub fn infinite_action_loop(
    privilege: &Privileges,
    rules: &mut Vec<Rule>,
    alerts: &mut Vec<Alert>,
) {
    loop {
        let welcome = menu();

        match welcome {
            Statement::Menu => continue,
            Statement::TrafficInterception => traffic_interception(rules, alerts),
            Statement::ComputerInformation => {
                let should_run = Arc::new(Mutex::new(true));
                let should_run_clone = Arc::clone(&should_run);

                thread::spawn(move || {
                    connection_start(should_run_clone);
                });
                let mut input = String::new();

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
            Statement::ComputerRulesChanging => println!("4"),
            Statement::Exit => {
                let _ = write_to_file(rules.to_vec());
                print_all_net_alert(alerts);
                process::exit(0)
            }
        }
    }
}

fn menu() -> Statement {
    println!("Please tell me what you want to do\n\n0. Menu again\n1. Traffic Interception\n2. Computer Information\n3. Network Rules Changing\n4. Computer Rules Changing\n5. Exit");
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
