use std::fs::File;
use std::io;
use std::io::{BufRead, Result};
use std::path::Path;
use regex::Regex;

fn main() {
    if let Ok(lines) = read_lines("/home/erik/.bash_history") {
        for line in lines {
            println!("{}", line.unwrap());
        }
    } else { println!("No history found"); }
}

fn get_packages(line: String) -> Result<Vec<String>> {
    let re = Regex::new(r"^pacman -S (.+)$").unwrap();
    let mut packages = Vec::new();
    let Some(captures) = re.captures(&*line);

    Ok(packages)
}

fn read_lines<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}