use crate::prelude::*;
use hex;
use sha3::{Digest, Sha3_256};

#[derive(Debug)]
pub enum Privileges {
    Admin,
    Viewer,
    Slave,
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
    let s = String::from("Not found that username");

    let res_creds: Vec<String> = match creds {
        s => {
            panic!("Not found that username, terminate..");
        }
        _ => {
            let ind = creds.find("@").unwrap();
            let before = &creds[..ind];
            let after = &creds[ind + 1..];

            vec![String::from(before), String::from(after)]
        }
    };

    let write = "Enter your password".truecolor(193, 251, 222).on_purple();
    println!("{}", write);
    io::stdin().read_line(&mut password);
    let mut hasher = Sha3_256::new();
    hasher.update(password.as_bytes());
    let password = hasher.finalize();
    let password = hex::encode(password);

    if res_creds[1] == password && name == "admin" {
        return Privileges::Admin;
    } else if res_creds[1] == password {
        return Privileges::Viewer;
    }
    Privileges::Slave
}
