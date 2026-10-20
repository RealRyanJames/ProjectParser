use std::{fs::OpenOptions, io::Write};

pub fn create(message: String) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("app.txt")?;

    file.write(String::from(message).as_bytes())?;

    Ok(())
}
