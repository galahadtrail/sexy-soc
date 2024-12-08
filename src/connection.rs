use crate::prelude::*;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;

#[derive(Eq, PartialEq)]
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
    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer).unwrap();
    *response_from_host.lock().unwrap() = String::from(str::from_utf8(&buffer).unwrap());
}

pub fn connection_start(
    should_run: Arc<Mutex<bool>>,
    hash_rules: &str,
    hash_alerts: Arc<Mutex<Vec<ComputerAlert>>>,
) {
    // Создаем TCP слушатель на порту 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
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
                let response_clone = Arc::clone(&response);
                thread::spawn(move || {
                    handle_client(stream, hash_rules_clone, response_clone);
                });
                if *response.lock().unwrap() != "Empty" {
                    let alerts: Vec<String> = response
                        .lock()
                        .unwrap()
                        .split("@")
                        .map(String::from)
                        .collect();
                    hash_alerts.lock().unwrap().push(ComputerAlert {
                        ip: host_ip,
                        paths: alerts,
                    });
                }
            }
            Err(_e) => {
                return;
            }
        }
    }
}
