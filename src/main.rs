use regex::{Captures, Regex};
use std::fs::File;
use std::io;
use std::io::{BufRead, Result};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("/home/erik/.bash_history") {
        for line in lines {
            let line_result = get_package_from_line(line.unwrap());

            if let Ok(packages) = line_result {
                println!("Package: {}", packages.join(""));
            }
        }
    } else {
        println!("No history found");
    }
}

fn get_package_from_line(line: String) -> Result<Vec<String>> {
    let re = Regex::new(r"^(sudo\s+)?(pacman|yay)\s+-S (?<packages>.+)$").unwrap();
    let mut packages: Vec<String> = Vec::new();

    let Some(captures): Option<Captures> = re.captures(&line) else {
        return Err(io::Error::new(io::ErrorKind::Other, "Invalid line"));
    };

    packages.push(captures["packages"].parse().unwrap());

    Ok(packages)
}

fn read_lines<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
