// the outline of the algorithm in 4-digit game:
// variables:
// numbers := a set of all possible numbers initialized at the beginning of the game attempt with a constraint of different digits
// numbers = {0000, 0001, 0002, ..., 9999}
// remove all numbers that do not satisfy the constraint of different digits
// numbers_probabilities := a map of numbers and their probabilities initialized at the beginning of the game attempt
// numbers_probabilities = 
// past_guesses := a set of all past guesses expressed as tuples (guess, (bulls, cows))
// past_guesses = {}

use std::collections::HashMap;
use std::collections::HashSet;
use rand::seq::SliceRandom;
pub struct BaselineAlgo {
    numbers : Vec<String>,
    past_guesses : Vec<(String, (usize, usize))>
}

impl BaselineAlgo {
    pub fn new(numbers: Vec<String>) -> BaselineAlgo {
        let mut numbers = numbers;
        let mut past_guesses = Vec::new();
        return BaselineAlgo {numbers, past_guesses}; 
    }

    pub fn get_numbers(&self) -> &Vec<String> {
        return &self.numbers;
    }

    pub fn get_past_guesses(&self) -> &Vec<(String, (usize, usize))> {
        return &self.past_guesses;
    }

    pub fn guess(&mut self) -> String {
        let mut best_guess = String::new();
        best_guess = self.numbers.choose(&mut rand::thread_rng()).unwrap().to_string();
        // we shold remove the guess from the set of numbers and add the guess to the set of past guesses
        return best_guess;
    }

    // temporary function
    pub fn add_guess(&mut self, guess: String, bulls: usize, cows: usize) {
        self.past_guesses.push((String::from(guess), (bulls, cows)));
    }

    pub fn find_valid_numbers(&self) -> Vec<String> {
        let mut valid_numbers = Vec::new();
        for number in self.numbers.iter() {
            let mut valid = true;
            for (past_guess, (past_bulls, past_cows)) in self.past_guesses.iter() {
                let local_bulls = number.chars().zip(past_guess.chars()).filter(|(a, b)| {
                    a == b
                }).count();
                let intersection_cardinality = number.chars().filter(|&c| {
                    past_guess.contains(c)
                }).count();    
                // if the number does not share some properties with the past guesses we discard it
                // the question is weather we may use stronger conditions here
                if number == past_guess || local_bulls < *past_bulls || intersection_cardinality - *past_bulls < *past_cows {
                    valid = false;
                    break;
                }
            }
            if valid {
                valid_numbers.push(number.clone());
            }
        }
        return valid_numbers;
    }
    
    pub fn update_numbers(&mut self) {
        self.numbers = self.find_valid_numbers();
    }

}

pub fn generate_default_init_values_for_numbers() -> Vec<String> {
    let mut numbers : Vec<(String)> = Vec::new();
    let tmp_numbers : Vec<String> = (0..10000).map(|u| -> String {format!("{:04}", u)}).collect::<Vec<String>>();
    for number in  tmp_numbers {
        let mut valid = true;
        // check if the number has different digits
        let mut digits = HashSet::new();
        for digit in number.chars() {
            if digits.contains(&digit) {
                valid = false;
                break;
            } else {
                digits.insert(digit);
            }
        }
        if valid {
            numbers.push(number);
        }            
    }
    return numbers;
}

// fn main() {
//     let mut defaults : Vec<String> = generate_default_init_values_for_numbers();
//     let mut ba = BaselineAlgo::new(defaults);
//     // println!("{}", ba.guess());

// }


// additional comments
// if we want to incorporate the knowledge about the distribution of the numbers from which
// the humans would draw when asked to think about a number we may take a representative sample
// of human guesses (the best scenario would be when we take one number from one person, but I guess
// we may safely assume that the numbers drawn from just one person are "quite independent")
// and then we may use the sample to estimate the distribution of the numbers
// even better would be to take a sample of numbers from a group of people who are good bulls and cows
// players, so that the ease of guessing a certain number is somehow incorporated in the distribution of the numbers
// the best case would be to have at disposition a sample from the opponents number distribution
// here we may try to learn the distribution of the numbers from multiple games with the same opponnent assuming his dist is stationary