use std::fs::File;
use std::io::{self, BufRead};
use rand::prelude::*;

pub fn get_word() -> String
{
    let mut suggestions: Vec<String> = Vec::new();
    let file_name: String = String::from("words.txt");
    let mut _rng = rand::thread_rng();

    if let Ok(lines) = read_lines(file_name)
    {
        for line in lines 
        {
            match line
            {
                Ok(line) => suggestions.push(line),
                Err(_) => println!("Error while reading")
            }
        }
    }

    let y = _rng.gen_range(0, suggestions.len());
    let reply = String::from(&suggestions[y]);
    reply
}

fn read_lines(filename: String) -> io::Result<io::Lines<io::BufReader<File>>>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}