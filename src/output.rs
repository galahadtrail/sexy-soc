use colored::*;
use std::{thread, time::Duration};

pub fn print_hello_message() {
    let mut prnt = "  /$$$$$$$  /$$$$$$  /$$   /$$ /$$   /$$          /$$$$$$$  /$$$$$$   /$$$$$$$"
        .red()
        .italic()
        .on_yellow();
    thread::sleep(Duration::from_millis(1000));
    println!("{}", prnt);
}
