#![allow(unused_assignments)]

use std::io;

fn main() {
    println!("Please input your text.");

    let mut text = String::new();
    io::stdin().read_line(&mut text)
        .expect("Failed to read line");

    println!("Original text: {}", text);

    let vowels = String::from("aeiouy");

    let mut pig_text = String::new();

    for word in text.split_whitespace() {
        let mut pig_word = String::new();
        let first_letter = word.chars().next().unwrap();
        if vowels.contains(first_letter) {
            pig_word = word.to_string();
            pig_word.push_str("-hay");
        } else {
            let mut chars: Vec<char> = word.chars().collect();
            let letter = chars.remove(0);
            chars.push('-');
            chars.push(letter);
            chars.push('a');
            chars.push('y');
            pig_word = chars.into_iter().collect();
        }

        pig_text.push_str(&pig_word);
        pig_text.push(' ');
    }
    println!("Pig text: {}", pig_text);
}
