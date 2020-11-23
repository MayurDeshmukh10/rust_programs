use std::io::{Write};


fn main() {
    let number_of_attempts = 10;
    
    let input_word = "Mayur";

    play_game(input_word.to_string(), number_of_attempts);
}


fn play_game(word: String, max_attempts: i32) {
    println!("\nWelcome to hangman! You have {} attempts!", max_attempts);
    
    let initial_word = word.clone();

    let mut attempts = 0;
    let mut word = init_word(initial_word.len());
    
    while attempts <= max_attempts {
        display_word(&word);
        update_word(&initial_word, &mut word, &mut attempts);

        if check_game_solved(&initial_word, &word) {
            println!("\nYou win! You took {} attempts for the word \"{}\"!", attempts, initial_word);
            break;
        }

        if attempts > max_attempts {
            println!("\nSorry, but you exceeded the maximum number of attempts \"{}\". Try next time!", initial_word);
            break;
        }
    }

    println!("\nThank you for playing Hangman!\n");
}

fn init_word(len: usize) -> String {
    let mut word = String::with_capacity(len);

    for _ in 0..len {
        word.push('_');
    }

    word
}


fn display_word(word: &str) {
    for c in word.chars() {
        print!("{} ", c);
    }
    println!();
}


fn update_word(initial_word: &str, word: &mut String, attempts: &mut i32) {
    let input_char = get_char("\nEnter your guess: ");

    let mut idx = 0;
    
    for c in initial_word.chars() {
        if c == input_char {
            word.remove(idx);
            word.insert(idx, input_char);
        }
        idx += 1;
    }

    *attempts += 1;
}

fn get_char(prompt: &str) -> char {
    print!("{}", prompt);

    std::io::stdout().flush().unwrap();
    
    let mut input = String::new();

    std::io::stdin().read_line(&mut input)
        .expect("no char!");

    input.trim()
        .chars()
        .nth(0)
        .unwrap()
}


fn check_game_solved(random_word: &str, word: &str) -> bool {
    random_word == word
}
