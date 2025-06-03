use std::fs::File;
use std::io::{self, BufRead};
use rand::Rng;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of words to generate
    #[arg(short = 'n', default_value_t = 4)]
    number: usize,
    /// Delimiter between words
    #[arg(short = 'd', default_value = "")]
    delimiter: String,
}

fn build_password(count: usize, delimiter: &str) -> io::Result<String> {
    // Look for wordlist in multiple locations
    let wordlist_paths = [
        "data/wordlist.txt",
        "/usr/share/pgen/wordlist.txt",
        "/etc/pgen/wordlist.txt",
    ];

    let mut file = None;
    for path in &wordlist_paths {
        if let Ok(f) = File::open(path) {
            file = Some(f);
            break;
        }
    }

    let file = file.ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "Wordlist file not found")
    })?;

    let words: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.trim().is_empty())
        .collect();

    if words.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "No words found in wordlist"));
    }

    let mut rng = rand::rng();
    let digit_position = rng.random_range(0..count);

    let selected: Vec<String> = (0..count)
        .map(|i| {
            let mut word = words[rng.random_range(0..words.len())].clone();
            // Randomly capitalize (50% chance)
            if rng.random_bool(0.5) {
                if let Some(first_char) = word.chars().next() {
                    let capitalized = first_char.to_uppercase().collect::<String>();
                    word = capitalized + &word[first_char.len_utf8()..];
                }
            }
            if i == digit_position {
                word.push_str(&rng.random_range(0..10).to_string());
            }
            word
        })
        .collect();

    Ok(selected.join(delimiter))
}

fn main() {
    let args = Args::parse();
    match build_password(args.number, &args.delimiter) {
        Ok(result) => println!("{}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}