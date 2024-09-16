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

    let Some(captures): Option<Captures> = re.captures(&line) else {
        return Err(io::Error::new(io::ErrorKind::Other, "Invalid line"));
    };

    let li: String = captures["packages"].parse().unwrap();
    let packages: Vec<String> = li.split_whitespace().map(|w| w.to_string()).collect();

    Ok(packages)
}

fn read_lines<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_package_from_line_should_return_package() {
        let result = get_package_from_line("sudo pacman -S firefox".to_string()).unwrap();

        assert_eq!(&result[0], "firefox");
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn get_package_from_line_should_return_package_when_several() {
        let result =
            get_package_from_line("sudo pacman -S firefox git lazygit neovim".to_string()).unwrap();

        assert_eq!(&result[1], "git");
        assert_eq!(&result[2], "lazygit");
        assert_eq!(result.len(), 4);
    }
}
