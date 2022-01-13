Author: Jake Huneau (jakehuneau@yahoo.com)

Rust script for solving [Wordle](https://www.powerlanguage.co.uk/wordle/).

## Instructions

To run, you need to [install Rust](https://www.rust-lang.org/tools/install), then run with `cargo run`.

Once run, script will give you a word to try. After trying, you will need to tell say which letters are correct. Incorrect letters are types as ".", letters that are correct but in the wrong place are typed lowercase, and correct letters in the correct location are given in uppercase.

For example, if the word is APPLE and you guess COPAY, then you will give the result as "..Pa.".

## Algorithm

The program first reads all 5-letter English words. It then determines the best words by determining a score for each word, and giving the word with the highest score to the user. After the user says which letters are correct, it filters the list of words to remove any that are no longer condidates. It continues this until the correct word is given.

### Scoring
A word is scored based primarily on letter frequency. A letter frequency table is built for each set of candidate words by finding how frequent each letter appears at each place in the words. A word is then given points based on that frequency table. Duplicate letters are a word are penalized by being given less points for the duplicates.


## Testing

Currently, it correctly guesses a word in 4.7 tries and is correct within 6 tries 89% of the time. The most difficult word is EALES with it taking 14 tries.
