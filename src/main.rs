use std::fs::File;
use std::io::{self, BufRead};

// Parse very simple configuration files of the form key=value

// don't use io::Result here, just use the standard Result
fn main() -> Result<(), String> {
    let filename = "test.ini";
    let file = File::open(filename).map_err(|e| e.to_string())?;
    for l in io::BufReader::new(file).lines() {
        let line = l.map_err(|e| e.to_string())?;
        if line.is_empty() {
            continue;
        }
        // this is actually the example from the stdlib
        // https://doc.rust-lang.org/std/primitive.str.html#method.split_once
        let (key, val) = line
            .split_once('=')
            .ok_or_else(|| "invalid config".to_string())?;

        // clippy complains here but let's just assume we wanted to match more stuff, so don't
        // remove the match
        #[allow(clippy::single_match)]
        match key {
            "foo" => {
                #[allow(clippy::blacklisted_name)]
                let foo = val;
                println!("{}", foo);
            }
            _ => {}
        }
        // Debug line
    }
    Ok(())
}
