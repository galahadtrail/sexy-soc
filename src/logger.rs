use chrono::{DateTime, Utc};
use std::fs::OpenOptions;
use std::io::{self, Write};

fn gimme_date_and_time() -> DateTime<Utc> {
    let current_date_time = Utc::now();
    current_date_time
}

pub fn write_current_dt_to_log(path: &str, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true) // Добавление данных в конец файла
        .open(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
