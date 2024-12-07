use crate::prelude::*;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;

fn handle_client(mut stream: TcpStream) {
    // Отправляем сообщение клиенту
    let msg = "Hello from server!";
    stream.write(msg.as_bytes()).unwrap();

    // Читаем ответ от клиента
    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer).unwrap();
    let response = str::from_utf8(&buffer).unwrap();
    println!("Received from client: {}", response);
}

pub fn connection_start(should_run: Arc<Mutex<bool>>) {
    // Создаем TCP слушатель на порту 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server is listening on port 7878");

    for stream in listener.incoming() {
        if !*should_run.lock().unwrap() {
            println!("Сервер завершает работу.");
            break;
        }

        match stream {
            Ok(stream) => {
                // Создаем новый поток для обработки клиента
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(_e) => {
                return;
            }
        }
    }
}
