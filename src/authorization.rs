use crate::prelude::*;
use hex;
use sha3::{Digest, Sha3_256};

pub enum Privileges {
    Admin,
    Viewer,
}

pub fn authorize() -> Privileges {
    let mut name = String::new();
    let mut password = String::new();

    let write = "Enter your name:".truecolor(193, 251, 222).on_purple();
    println!("{}", write);
    io::stdin().read_line(&mut name);
    let mut hasher = Sha3_256::new();
    hasher.update(name.as_bytes());
    let name = hasher.finalize();
    let name = hex::encode(name);
    println!("{}", name);
    if name == "8ac76453d769d4fd14b3f41ad4933f9bd64321972cd002de9b847e117435b08b".to_string() {
        println!("1");
    }

    let write = "Enter your password".truecolor(193, 251, 222).on_purple();
    println!("{}", write);
    io::stdin().read_line(&mut password);
    let mut hasher = Sha3_256::new();
    hasher.update(password);
    let password = hasher.finalize();

    Privileges::Admin
}
