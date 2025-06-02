use std::fs::File;
use std::io::{self, BufRead};
use rand::Rng;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of words to generate
    #[arg(short = 'n', default_value_t = 3)]
    number: usize,

    /// Delimiter between words
    #[arg(short = 'd', default_value = " ")]
    delimiter: String,
}


fn build_password(count: usize, delimiter: &str) -> io::Result<String> {
    let file = File::open("data/wordlist.txt")?;
    let words: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect();

    let mut rng = rand::rng();
    let digit_position = rng.random_range(0..count);

    let selected: Vec<String> = (0..count)
        .map(|i| {
            let mut word = words[rng.random_range(0..words.len())].clone();

            // Randomly capitalize (50% chance)
            if rng.random_bool(0.5) {
                if let Some(first) = word.get_mut(0..1) {
                    first.make_ascii_uppercase();
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
    let _args = Args::parse();
    match build_password(5, "-") {
        Ok(result) => println!("{}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}