use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::Read;
use std::path::Path;

extern crate rosalind;
use rosalind::rear::solve;

fn main() -> io::Result<()> {
    let input = get_input()?;
    println!("{}", solve(input));
    Ok(())
}

fn get_input() -> Result<Vec<String>, io::Error> {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let mut file = File::open(&path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(String::from(line))
            }
        }).collect())
}
