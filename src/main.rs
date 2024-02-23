use ansi_term::Colour::Green;
use anyhow::Result;
use std::io;
use clearscreen::clear;
use rand::seq::SliceRandom;

fn main() -> Result<()> {
    println!("Welcome to fallout terminal!\nPlease enter your words, and when you're done press <Enter>: ");
    let words = get_words()?;
    let mut is_running = true;
    let mut potential_words = words.clone();
    let mut selected_word: String;
    while is_running {
        clear()?;
        selected_word = potential_words.choose(&mut rand::thread_rng()).unwrap().to_string();
        show_words(&words, &potential_words, &selected_word);
        println!("Please click the highlighted word in your game, and enter its likeness:");
        let likeness = {
            let mut likeness = String::new();
            io::stdin().read_line(&mut likeness)?;
            likeness.trim().parse::<u32>()?
        };
        potential_words = filter_by_likeness(&mut potential_words, &selected_word, &likeness);
        if potential_words.len() == 1 {
            println!("The word is: {}", potential_words[0]);
            is_running = false;
        }
    }
    Ok(())
}

fn get_words() -> Result<Vec<String>> {
    let mut words: Vec<String> = Vec::new();
    let mut tmp_word = String::from("");
    while tmp_word != "\n" {
        tmp_word.clear();
        io::stdin().read_line(&mut tmp_word)?;
        words.push(tmp_word.trim().to_string());
    }
    Ok(words[..words.len()-1].to_vec())
}

fn show_words(words: &Vec<String>, potential_words: &Vec<String>, selected_word: &String) {
    for word in words {
        if word == selected_word {
            println!("{}", Green.underline().paint(word));
            continue;
        }
        else if potential_words.contains(word) {
            println!("{}", Green.paint(word));
            continue;
        }
        println!("{}", word);
    }
}

fn filter_by_likeness(potential_words: &mut Vec<String>, selected_word: &String, likeness: &u32) -> Vec<String> {
    let mut filtered_words: Vec<String> = Vec::new();
    for word in potential_words {
        let mut matching = 0;
        for (char_index, character) in word.chars().enumerate() {
            if selected_word.chars().nth(char_index) == Some(character) {
                matching += 1;
            }
        }
        if matching == *likeness {
            filtered_words.push(word.to_string());
        }
    }
    filtered_words
}
