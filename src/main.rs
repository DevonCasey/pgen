use std::fs::File;
use std::io::{self, BufRead};
use rand::Rng;
use clap::Parser;
use arboard::Clipboard;
use std::{thread, time::Duration};

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
        return Err(io::Error::new(io::ErrorKind::InvalidData, "No words found in the wordlist"));
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
    let password = build_password(4, "-")?;
    copy_to_clipboard_with_timeout(password, 45)?;
    Ok(())
}