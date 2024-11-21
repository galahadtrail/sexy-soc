use crate::prelude::*;
use hex;
use sha3::{Digest, Sha3_256};

pub enum Privileges {
    Admin,
    Viewer,
}

fn gimme_creds(name: &String) -> Result<String, Error> {
    let input = File::open("src/users/accounts.txt")?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        if name == line.as_ref().unwrap() {
            return Ok(line.unwrap().clone());
        }
    }
    return Ok(String::from("Not found that username"));
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

    let creds = match gimme_creds(&name) {
        Ok(res) => res,
        Err(e) => panic!("{}", e),
    };

    let write = "Enter your password".truecolor(193, 251, 222).on_purple();
    println!("{}", write);
    io::stdin().read_line(&mut password);
    let mut hasher = Sha3_256::new();
    hasher.update(password.as_bytes());
    let password = hasher.finalize();
    let password = hex::encode(password);

    Privileges::Admin
}
