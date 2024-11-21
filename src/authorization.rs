use crate::prelude::*;
use sha3::{Digest, Sha3_256};

pub enum Privileges {
    Admin,
    Viewer,
}

pub fn authorize() -> Privileges {
    let mut hasher = Sha3_256::new();
    let mut name = String::new();
    let mut password = String::new();
    let hello = "Enter your name:".truecolor(193, 251, 222).on_purple();

    println!("{}", hello);
    Privileges::Admin
}
