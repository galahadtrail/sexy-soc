use crate::prelude::*;

enum Privileges {
    Admin,
    Viewer,
}

pub fn authorize() -> Privileges {
    Privileges::Admin
}
