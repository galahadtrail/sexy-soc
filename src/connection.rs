use crate::prelude::*;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;

#[derive(Eq, PartialEq, Debug)]
pub struct ComputerAlert {
    ip: String,
    paths: Vec<String>,
}

fn handle_client(
    mut stream: TcpStream,
    hash_rules: String,
    response_from_host: Arc<Mutex<String>>,
) {
    // Отправляем сообщение клиенту
    stream.write(hash_rules.as_bytes()).unwrap();

    // Читаем ответ от клиента
    let mut buffer = [0; 4096];
    let bytes_read = stream.read(&mut buffer).unwrap();
    let response_str = str::from_utf8(&buffer[..bytes_read]).unwrap_or("");
    *response_from_host.lock().unwrap() = String::from(response_str);
}

pub fn connection_start(
    should_run: Arc<Mutex<bool>>,
    hash_rules: &str,
    hash_alerts: Arc<Mutex<Vec<ComputerAlert>>>,
) {
    // Создаем TCP слушатель на порту 7878
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let mut str = "Server is listening on port 7878"
        .truecolor(193, 251, 222)
        .on_purple();
    println!("{}", str);
    let hash_rules_owned = hash_rules.to_string();

    for stream in listener.incoming() {
        if !*should_run.lock().unwrap() {
            str = "Server've stopped".truecolor(193, 251, 222).on_purple();
            println!("{}", str);
            break;
        }

        match stream {
            Ok(stream) => {
                // Создаем новый поток для обработки клиента
                let hash_rules_clone = hash_rules_owned.clone();
                let response = Arc::new(Mutex::new(String::new()));
                let host_ip = stream.local_addr().unwrap().to_string();
                let responce_clonable = Arc::clone(&response);

                let handle = thread::spawn(move || {
                    let response_clone = Arc::clone(&response);
                    handle_client(stream, hash_rules_clone, response_clone);
                });
                handle.join().unwrap();
                if *responce_clonable.lock().unwrap() != "Empty" {
                    let alerts: Vec<String> = responce_clonable
                        .lock()
                        .unwrap()
                        .split("@")
                        .map(String::from)
                        .collect();
                    println!("{}", responce_clonable.lock().unwrap());
                    let new_alert = ComputerAlert {
                        ip: host_ip,
                        paths: alerts,
                    };
                    let _ = write_current_dt_to_log(
                        "logs/hash_alerts.log",
                        "success",
                        &format!("Achtung! {:?}", new_alert),
                    );
                    println!("Achtung! {:?}", new_alert);
                    hash_alerts.lock().unwrap().push(new_alert);
                }
            }
            Err(_e) => {
                return;
            }
        }
    }
}

pub fn print_all_hosts_alerts(hash_alerts: Arc<Mutex<Vec<ComputerAlert>>>) {
    let alerts = hash_alerts.lock().unwrap();
    for alert in alerts.iter() {
        println!("{:?}", alert);
    }
}
