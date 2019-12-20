use std::io::{self, Read, Write};
use std::env;

fn main() -> io::Result<()> {
    let secret = env::args().nth(1).unwrap_or(String::new());
    let mut file = Vec::new();

    eprintln!(r#"secret = "{}";"#, secret);

    io::stdin().read_to_end(&mut file)?;

    let secret = secret.as_bytes().iter().cycle();

    let result: Vec<u8> = file.iter()
        .zip(secret)
        .map(|(file, secret)| file ^ secret)
        .collect();

    io::stdout().write_all(&result)?;
    Ok(())
}
