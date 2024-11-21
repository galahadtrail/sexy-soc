use crate::prelude::*;
use hex;
use sha3::{Digest, Sha3_256};

#[derive(Debug)]
pub enum Privileges {
    Admin,
    Viewer,
}

fn gimme_creds(name: &String) -> Result<String, Error> {
    let input = File::open("src/users/accounts.txt")?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        let slice_line = line.as_ref().unwrap();
        let ind = slice_line.find("@").unwrap();
        let before = &slice_line[..ind];

        if name == before {
            return Ok(line.unwrap().clone());
        }
    }
    return Ok(String::from("Not found that username"));
}

pub fn authorize() -> Result<Privileges, Error> {
    let mut name = String::new();
    let mut password = String::new();

    let write = "Enter your name:".truecolor(193, 251, 222).on_purple();
    println!("{}", write);
    let _ = io::stdin().read_line(&mut name);

    let mut hasher = Sha3_256::new();
    hasher.update(name.trim().as_bytes());
    let name = hasher.finalize();
    let name = hex::encode(name);

    let creds = match gimme_creds(&name) {
        Ok(res) => res,
        Err(e) => panic!("{}", e),
    };

    let ind = creds
        .find("@")
        .expect("Ooops, something went wrong with username..");
    let before = &creds[..ind];
    let after = &creds[ind + 1..];

    let res_creds = vec![String::from(before), String::from(after)];

    let write = "Enter your password".truecolor(193, 251, 222).on_purple();
    println!("{}", write);
    let _ = io::stdin().read_line(&mut password);

    let mut hasher = Sha3_256::new();
    hasher.update(password.as_bytes());
    let password = hasher.finalize();
    let password = hex::encode(password);

    let name = String::from("admin");
    let mut hasher = Sha3_256::new();
    hasher.update(name.as_bytes());
    let name = hasher.finalize();
    let name = hex::encode(name);

    if res_creds[1] == password && name == res_creds[0] {
        return Ok(Privileges::Admin);
    } else if res_creds[1] == password {
        return Ok(Privileges::Viewer);
    }

    panic!("Invalid credentials")
}
