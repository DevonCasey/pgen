use std::fs::File;
use std::io::{self, BufRead, BufReader};
use rand::Rng;
use clap::Parser;
use arboard::Clipboard;
use std::{thread, time::Duration};

// Include the fallback wordlist that's embedded in the binary
static DEFAULT_WORDLIST: &str = include_str!("../data/wordlist.txt");

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of words to generate
    #[arg(short = 'n', default_value_t = 4)]
    number: usize,
    /// Delimiter between words
    #[arg(short = 'd', default_value = "-")]
    delimiter: String,
}

fn build_password(count: usize, delimiter: &str) -> io::Result<String> {
    // Look for wordlist in multiple locations
    let wordlist_paths = [
        "data/wordlist.txt",
        "/usr/share/pgen/wordlist.txt",
        "/etc/pgen/wordlist.txt",
        "./wordlist.txt"
    ];

    // Try to open the wordlist from potential locations
    let mut words = Vec::new();
    for path in &wordlist_paths {
        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(word) = line {
                    let trimmed = word.trim();
                    if !trimmed.is_empty() {
                        words.push(trimmed.to_string());
                    }
                }
            }
            break;
        }
    }
    // If no external wordlist found, use the embedded one
    if words.is_empty() {
        words = DEFAULT_WORDLIST
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| line.to_string())
            .collect();
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

fn copy_to_clipboard_with_timeout(password: String, timeout_secs: u64) -> io::Result<()> {
    // Initialize clipboard
    let mut clipboard = Clipboard::new()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    // Copy password to clipboard
    clipboard.set_text(password)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    println!("Password copied to clipboard. The clipboard will clear in {} seconds.", timeout_secs);

    // Spawn a thread to clear the clipboard after timeout
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(timeout_secs));
        if let Ok(mut clipboard) = Clipboard::new() {
            let _ = clipboard.set_text("");
        }
    });

    Ok(())
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let password = build_password(args.number, &args.delimiter)?;
    copy_to_clipboard_with_timeout(password, 45)?;
    Ok(())
}