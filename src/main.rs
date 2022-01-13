use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::io::stdin;
use std::env;

fn main() {
    // Check if a word is given to run automatically against
    let word = match env::args().nth(1) {
        Some(arg) => arg,
        None => String::from(""),
    };

    // Read all 5 letter english words
    let words_string = read_to_string("words.txt").unwrap();
    let mut words: Vec<&str> = words_string.lines().collect();
    let mut known_letters: [char; 5] = [' '; 5]; // Positions of letters we know
    let mut known_wrong_letters: HashSet<char> = HashSet::new(); // Letters that aren't anywhere
    let mut known_wrong_locations: [HashSet<char>; 5] = [
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
    ]; // Letters we know aren't in a location
    let mut guess: &str; // proposed solution
    for i in 0..20 {
        // Don't allow double letters on first try to get more letters
        guess = pick_word(&words);
        if guess == word {
            // If we are doing automated mode and get the word then print the number of guesses it took
            println!("{:?} {}", guess, i + 1);
            return;
        }

        if word.is_empty() {
            println!("Try {:?} [{} possible words]", guess, words.len());
        }

        for (i, &letter) in get_result(&word, guess).iter().enumerate() {
            if letter != ' ' {
                if is_uppercase(letter) {
                    // Correct spot
                    known_letters[i] = letter;
                    for j in 0..5 {
                        known_wrong_locations[j].remove(&letter);
                    }
                } else {
                    // Misplaced
                    known_wrong_locations[i].insert(to_uppercase(letter));
                }
            } else {
                known_wrong_letters.insert(guess.chars().nth(i).unwrap());
            }
        }
        words = filter_words(
            words,
            known_letters,
            known_wrong_letters.clone(),
            known_wrong_locations.clone(),
        );
    }
}

/// Checks if a letter is uppercase
/// letters A-Z as u32 are 65-90 and a-z are 97-122
fn is_uppercase(letter: char) -> bool {
    letter as u32 <= 90
}

/// Converts a lowercase char to uppercase
fn to_uppercase(letter: char) -> char {
    std::char::from_u32(letter as u32 - 32).unwrap()
}

/// Converts an uppercase char to lowercase
fn to_lowercase(letter: char) -> char {
    std::char::from_u32(letter as u32 + 32).unwrap()
}

/// Gets user input for correct letters
fn get_result(word: &String, guess: &str) -> [char; 5] {
    if !word.is_empty() {
        // Automated
        let mut letters = [' '; 5];
        for (i, letter) in guess.chars().enumerate() {
            if word.chars().nth(i).unwrap() == letter {
                letters[i] = letter;
            } else if word.chars().collect::<Vec<char>>().contains(&letter) {
                letters[i] = to_lowercase(letter);
            }
        }
        let mut letter_count: HashMap<char, u8> = HashMap::new();
        for letter in word.chars() {
            if !letter_count.contains_key(&letter) {
                letter_count.insert(letter, 1);
            } else {
                letter_count.insert(letter, letter_count[&letter] + 1);
            }
        }
        let mut correct_letter_count: HashMap<char, u8> = HashMap::new();
        for letter in letters {
            if is_uppercase(letter) {
                if !correct_letter_count.contains_key(&letter) {
                    correct_letter_count.insert(letter, 1);
                } else {
                    correct_letter_count.insert(letter, correct_letter_count[&letter] + 1);
                }
            }
        }
        for i in 0..5 {
            if !is_uppercase(letters[i]) && correct_letter_count.contains_key(&to_uppercase(letters[i])) {
                if letter_count[&to_uppercase(letters[i])] == correct_letter_count[&to_uppercase(letters[i])] {
                    letters[i] = ' ';
                }
            }
        }
        letters
    } else {
        println!("Enter the result with misplaced letters in lowercase, letters in the correct place in uppercase, and incorrect letters with . (ex: .aE..): ");
        get_letters()
    }
}

/// Gets user input for letters
fn get_letters() -> [char; 5] {
    let mut input = String::new();
    let mut letters = [' '; 5];
    stdin().read_line(&mut input).expect("Couldn't read input");
    if input.trim().len() > 5 {
        println!("This is not a valid word. Try again:");
        return get_letters();
    }
    for i in 0..input.trim().len() {
        let curr_letter = input.chars().nth(i).unwrap();
        if curr_letter != '.' {
            letters[i] = curr_letter;
        }
    }
    letters
}

/// Filters words by removing any words that do not contain correct letters
fn filter_words(
    words: Vec<&str>,
    known_letters: [char; 5],
    known_wrong_letters: HashSet<char>,
    known_wrong_locations: [HashSet<char>; 5],
) -> Vec<&str> {
    // All the letters that are correct somewhere
    let mut misplaced_letters: HashSet<char> = HashSet::new();
    for wrong_letter_set in &known_wrong_locations {
        for &letter in wrong_letter_set {
            misplaced_letters.insert(letter);
        }
    }
    let mut curr_letter: char = ' ';
    words
        .into_iter()
        .filter(|word| {
            let mut contains_misplaced_letters: HashMap<char, bool> = HashMap::new();
            for &letter in &misplaced_letters {
                contains_misplaced_letters.insert(letter, false);
            }
            for i in 0..5 {
                curr_letter = word.chars().nth(i).unwrap();
                if known_letters[i] != ' ' {
                    if curr_letter != known_letters[i] {
                        return false;
                    }
                } else {
                    if known_wrong_locations[i].contains(&curr_letter) {
                        return false;
                    }
                    if misplaced_letters.contains(&curr_letter) {
                        contains_misplaced_letters.insert(curr_letter, true);
                    }
                    if !misplaced_letters.contains(&curr_letter)
                        & known_wrong_letters.contains(&curr_letter)
                    {
                        return false;
                    }
                }
            }
            for (_, has_letter) in contains_misplaced_letters {
                if !has_letter {
                    return false;
                }
            }
            true
        })
        .collect()
}

/// Finds the total count of each letter in each place
fn create_frequency_map(words: &Vec<&str>) -> [HashMap<char, u16>; 5] {
    let mut frequency_map = [
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
    ];
    for word in words {
        for (i, letter) in word.chars().enumerate() {
            if !frequency_map[i].contains_key(&letter) {
                frequency_map[i].insert(letter, 1);
            } else {
                frequency_map[i].insert(letter, frequency_map[i][&letter] + 1);
            }
        }
    }
    frequency_map
}

/// Word score is calcuated based on which has highest frequency letter in each spot
fn get_word_score(word: &str, frequency_map: &[HashMap<char, u16>; 5]) -> u16 {
    let mut score: u16 = 0;
    let mut seen_letters: HashSet<char> = HashSet::new();
    for (i, letter) in word.chars().enumerate() {
        if !seen_letters.contains(&letter) {
            // Only give points if this is first time seeing letter
            // penalize duplicate letters
            score += frequency_map[i][&letter];
        }
        seen_letters.insert(letter);
    }
    score
}

/// Picks the best word based on which has the highest score
fn pick_word<'a>(words: &'a Vec<&str>) -> &'a str {
    let frequency_map = create_frequency_map(words); // Create new frequency map for new set of words
    let mut best_word: &str = "";
    let mut best_word_score: u16 = 0;
    for word in words {
        let new_score = get_word_score(word, &frequency_map);
        if new_score >= best_word_score {
            best_word = word;
            best_word_score = new_score;
        }
    }
    best_word
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_words_works_with_no_known_letters() {
        let mut words: Vec<&str> = vec!["abcde", "abdce", "bcdef"];
        let known_letters: [char; 5] = [' ', ' ', ' ', ' ', ' '];
        words = filter_words(
            words,
            known_letters,
            HashSet::new(),
            [
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
            ],
        );
        assert_eq!(words, vec!["abcde", "abdce", "bcdef"]);
    }

    #[test]
    fn filter_words_works_first_letter() {
        let mut words: Vec<&str> = vec!["abcde", "abdce", "bcdef"];
        let known_letters: [char; 5] = ['a', ' ', ' ', ' ', ' '];
        words = filter_words(
            words,
            known_letters,
            HashSet::new(),
            [
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
            ],
        );
        assert_eq!(words, vec!["abcde", "abdce"]);
    }

    #[test]
    fn filter_words_works_middle_letter() {
        let mut words: Vec<&str> = vec!["abcde", "abdce", "bccef"];
        let known_letters: [char; 5] = [' ', ' ', 'c', ' ', ' '];
        words = filter_words(
            words,
            known_letters,
            HashSet::new(),
            [
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
            ],
        );
        assert_eq!(words, vec!["abcde", "bccef"]);
    }

    #[test]
    fn filter_words_works_multiple_letters() {
        let mut words: Vec<&str> = vec!["abcde", "abdce", "bccef"];
        let known_letters: [char; 5] = ['a', ' ', 'c', ' ', ' '];
        words = filter_words(
            words,
            known_letters,
            HashSet::new(),
            [
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
            ],
        );
        assert_eq!(words, vec!["abcde"]);
    }

    #[test]
    fn filter_words_works_misplaced_letter() {
        let mut words: Vec<&str> = vec!["abcde", "bcdea", "bccef"];
        let known_letters: [char; 5] = [' ', ' ', ' ', ' ', ' '];
        words = filter_words(
            words,
            known_letters,
            HashSet::new(),
            [
                HashSet::from_iter(vec!['a']),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
            ],
        );
        assert_eq!(words, vec!["bcdea"]);
    }

    #[test]
    fn filter_words_works_misplaced_letters_removes_all() {
        let mut words: Vec<&str> = vec!["abcde", "bcdea", "bccef"];
        let known_letters: [char; 5] = [' ', ' ', ' ', ' ', ' '];
        words = filter_words(
            words,
            known_letters,
            HashSet::new(),
            [
                HashSet::from_iter(vec!['z']),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
            ],
        );
        let should_equal: Vec<&str> = vec![];
        assert_eq!(words, should_equal);
    }

    #[test]
    fn filter_words_works_misplaced_multiple_letters() {
        let mut words: Vec<&str> = vec!["abcde", "zbbde", "abbce"];
        let known_letters: [char; 5] = [' ', ' ', ' ', ' ', ' '];
        words = filter_words(
            words,
            known_letters,
            HashSet::new(),
            [
                HashSet::from_iter(vec!['b']),
                HashSet::new(),
                HashSet::new(),
                HashSet::from_iter(vec!['c']),
                HashSet::new(),
            ],
        );
        assert_eq!(words, vec!["abcde"]);
    }

    #[test]
    fn filter_words_works_known_wrong_letters() {
        let mut words: Vec<&str> = vec!["abcde", "zbcde", "abbbef"];
        let known_letters: [char; 5] = [' ', ' ', ' ', ' ', ' '];
        words = filter_words(
            words,
            known_letters,
            HashSet::from_iter(vec!['z']),
            [
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
            ],
        );
        assert_eq!(words, vec!["abcde", "abbbef"]);
    }

    #[test]
    fn filter_words_works_letter_in_wrong_spot() {
        let mut words: Vec<&str> = vec!["abcde", "zbcde", "abbbf"];
        let known_letters: [char; 5] = [' ', ' ', ' ', ' ', ' '];
        words = filter_words(
            words,
            known_letters,
            HashSet::new(),
            [
                HashSet::new(),
                HashSet::from_iter(vec!['a']),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
            ],
        );
        assert_eq!(words, vec!["abcde", "abbbf"]);
    }

    #[test]
    fn is_uppercase_works() {
        let all_upper = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let all_lower = "abcdefghijklmnopqrstuvwxyz";

        for letter in all_upper.chars() {
            assert_eq!(is_uppercase(letter), true);
        }
        for letter in all_lower.chars() {
            assert_eq!(is_uppercase(letter), false);
        }
    }

    #[test]
    fn to_uppercase_works() {
        assert_eq!(
            "abcdefghijklmnopqrstuvwxyz"
                .chars()
                .map(|letter| to_uppercase(letter))
                .collect::<String>(),
            String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
        );
    }

    #[test]
    fn to_lowercase_works() {
        assert_eq!(
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
                .chars()
                .map(|letter| to_lowercase(letter))
                .collect::<String>(),
            String::from("abcdefghijklmnopqrstuvwxyz")
        );
    }
}
