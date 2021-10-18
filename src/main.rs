use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Parse very simple configuration files of the form key=value

fn main() -> std::io::Result<()> {
    let filename = "test.ini";
    let file = File::open(filename)?;
    for l in io::BufReader::new(file).lines() {
        let line = l?;
        if line.len() == 0 {
            continue;
        }
        if let Some(i) = line.find('=') {
            let key = &line[..i];
            let value = &line[i + 1..];
            match key {
                "foo" => {
                    let foo = value;
                    println!("{}", foo);
                }
                _ => {}
            }
            // Debug line
        } else {
            // Replace with whatever you want to do on malformed config lines
            panic!("Invalid config")
        }
    }
    Ok(())
}

