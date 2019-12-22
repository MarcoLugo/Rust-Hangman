use rand::Rng;
use colored::Colorize;
use crate::console_io;  // import sibling module

/// Dictionary of available words for the game. A future version could
/// retrieve them directly from an online dictionary via an API.
const WORDS: [&str; 6] = ["hangman", "plane", "programming", "seven", "world", "person"];


struct Board {
    word_to_guess: Vec<char>,
    board: Vec<char>,
    n_hidden_letters: usize,
    n_guesses: u32,
}

impl Board {
    pub fn new(word_to_guess: String) -> Board {
        Board {
            word_to_guess: word_to_guess.chars().collect(),
            board: vec!['_'; word_to_guess.len()],
            n_hidden_letters: word_to_guess.len(),
            n_guesses: 0
        }
    }

    /// Make a guess against the board using the guessed char.
    /// It will return false if all letters have been revealed.
    pub fn make_guess(&mut self, guess: char) -> bool {
        let mut correct_guess_indices = vec![];

        for (i, c) in self.word_to_guess.iter().enumerate() {
            if *c == guess && self.board[i] == '_' {
                self.board[i] = *c;
                self.n_hidden_letters -= 1;
                correct_guess_indices.push(i);
            }
        }
        self.n_guesses += 1;
        self.print_board_with_differences(correct_guess_indices);
        self.n_hidden_letters > 0  // is game over?
    }

    /// Prints the board to the console highlighting the right
    /// guesses to give feedback to the user.
    fn print_board_with_differences(&self, differences: Vec<usize>) {
        for (i, c) in self.board.iter().enumerate() {
            if differences.contains(&i) {
                let s = format!("{}", c);
                print!("{} ", s.green().bold());
            } else {
                print!("{} ", c);
            }
        }
        println!("\n");
    }

    /// Prints the board to the console.
    pub fn print_board(&self) {
        for c in self.board.iter() {
            print!("{} ", c);
        }
        println!("\n");
    }
}

/// Pick random word from the available words.
fn choose_word() -> String {
    let mut rng = rand::thread_rng();
    let word_index = rng.gen_range(0, WORDS.len()-1);
    String::from(WORDS[word_index])
}

/// Main loop for the game. Keeps asking for
/// guesses until the user finally discovers all
/// letters in the randomly picked word.
pub fn word_loop() {
    let mut user_alive = true;
    let mut guessed_letter: char;
    let word_to_guess = choose_word();

    let mut board = Board::new(word_to_guess);
    board.print_board();

    while user_alive {
        guessed_letter = console_io::get_letter();
        user_alive = board.make_guess(guessed_letter);
    }

    println!("Congratulations! You guessed the word after {} guesses.", board.n_guesses);
}