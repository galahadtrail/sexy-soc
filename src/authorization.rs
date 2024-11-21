use crate::prelude::*;
use sha3::{Digest, Sha3_256};

enum Privileges {
    Admin,
    Viewer,
}

pub fn authorize() -> Privileges {
    let mut hasher = Sha3_256::new();
    let mut name = String::new();
    let mut password = String::new();
    Privileges::Admin
}
