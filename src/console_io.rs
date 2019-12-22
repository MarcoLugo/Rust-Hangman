use std::io::prelude::*;
use std::io;


/// Removes trailing newline from string.
fn trim_newline(s: &mut String) {
    while s.ends_with('\n') || s.ends_with('\r') {
        s.pop();
    }
}

/// Shows welcome message on console.
pub fn welcome() {
    let welcome_bar = "=".repeat(40);
    println!("{}", welcome_bar);
    println!("                Hangman");
    println!("{}\n", welcome_bar);
}

/// Gets one letter from console input.
///
/// If the string is empty or larger than one letter,
///  it loops again until the input is a single letter.
pub fn get_letter() -> char {
    let mut guess = String::new();

    print!("Please enter a letter: ");
    io::stdout().flush().expect("Could not flush stdout");

    loop {
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        trim_newline(&mut guess);
        
        match guess.len() {
            0 => continue,
            1 => break,
            _ => {
                println!("Please enter a single letter. You entered: {}\n", guess);
                guess.clear();  // clear contents, otherwise read_line will append on user input
            }
        }
    }

    guess.chars().next().unwrap()  // return first character of string
}