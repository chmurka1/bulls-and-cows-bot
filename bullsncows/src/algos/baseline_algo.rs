use std::collections::HashSet;
use rand::seq::SliceRandom;

pub struct BaselineAlgo {
    numbers : Vec<String>,
    past_guesses : Vec<(String, (usize, usize))>,
    last_guess_updated : bool,
    ndigits : usize,
}

impl BaselineAlgo {
    pub fn new(numbers: Vec<String>) -> Option<BaselineAlgo> {
        if numbers.len() == 0 {
            return None;
        }

        let past_guesses: Vec<(String, (usize, usize))> = Vec::new();
        let last_guess_updated: bool = true;
        let ndigits: usize = numbers[0].len();
        return Some(BaselineAlgo {numbers, past_guesses, last_guess_updated, ndigits}); 
    }

    pub fn guess(&mut self) -> Option<String> {
        // written explicitly to handle exceptions differently in the future
        if self.last_guess_updated == false {
            return None;
        }
        if self.numbers.is_empty() {
            return None;
        }
        
        let mut best_guess: String = String::new(); 
        
        best_guess = self.numbers.choose(&mut rand::thread_rng()).unwrap().to_string();
        self.numbers.retain(|x: &String| x != &best_guess);
        self.past_guesses.push((best_guess.clone(), (usize::MAX, usize::MAX)));
        self.last_guess_updated = false;
        return Some(best_guess);
    }

    pub fn get_numbers_count(&self) -> usize {
        return self.numbers.len();
    }
    
    fn get_last_guess(&self) -> Option<(String, (usize, usize))> {
        if self.past_guesses.is_empty() {
            return None;
        }

        return Some(self.past_guesses.last().unwrap().clone());
    }

    pub fn incorporate_guess_feedback(&mut self, bulls: usize, cows: usize) -> Option<bool> {
        if bulls + cows > self.ndigits {
            return None;
        }
        else if self.last_guess_updated == true {
            return Some(false);
        }
        self.past_guesses.last_mut().unwrap().1 = (bulls, cows);
        self.numbers = self.find_valid_numbers(self.get_last_guess().unwrap()).unwrap();
            
        self.last_guess_updated = true;
        return Some(true);    
    }

    fn find_valid_numbers(&self, guess: (String, (usize, usize))) -> Option<Vec<String>> {
        if guess.0.len() != self.ndigits || guess.1.0 + guess.1.1 > self.ndigits {
            return None;
        }

        let mut valid_numbers:Vec<String> = Vec::new();
        for number in self.numbers.iter() {
            let local_bulls: usize = number.chars().zip(guess.0.chars()).filter(|(a, b)| {
                a == b
            }).count();
            let local_cows: usize = number.chars().filter(|&c| {
                guess.0.contains(c)
            }).count() - local_bulls;
            if local_bulls == guess.1.0 && local_cows == guess.1.1 {
                valid_numbers.push(number.clone());
            }
        }
    
    return Some(valid_numbers);
    }

    // just for testing purposes
    pub fn pub_get_last_guess(&self) -> Option<(String, (usize, usize))> {
        return self.get_last_guess();
    }

    // just for testing purposes
    fn pub_find_valid_numbers(&self, guess: (String, (usize, usize))) -> Option<Vec<String>> {
        return self.find_valid_numbers(guess);
    }

}

pub fn generate_default_init_values_for_numbers() -> Vec<String> {
    let mut numbers : Vec<String> = Vec::new();
    let tmp_numbers : Vec<String> = (0..10000).map(|u| -> String {format!("{:04}", u)}).collect::<Vec<String>>();
    for number in  tmp_numbers {
        let mut valid = true;
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
#[cfg(test)]
mod tests {
    use super::{generate_default_init_values_for_numbers, BaselineAlgo};
    #[test]
    fn test_generate_default_init_values_for_numbers_output() {
        let numbers = generate_default_init_values_for_numbers();
        assert_eq!(numbers.len(), 5040);
        let numbers = generate_default_init_values_for_numbers();
        for number in numbers {
            let mut digits = std::collections::HashSet::new();
            for digit in number.chars() {
                digits.insert(digit);
            }
            assert_eq!(digits.len(), 4);
        }
    }
    #[test]
    fn test_baseline_algo_new() {
        let numbers = generate_default_init_values_for_numbers();
        let ba = BaselineAlgo::new(numbers);
        assert_eq!(ba.as_ref().is_some(), true);
        assert!(ba.as_ref().unwrap().numbers.len() > 0);
        assert_eq!(ba.as_ref().unwrap().past_guesses.len(), 0);
        assert_eq!(ba.as_ref().unwrap().last_guess_updated, true);
        assert_eq!(ba.as_ref().unwrap().ndigits, 4);
        let numbers = generate_default_init_values_for_numbers();
        assert_eq!(ba.as_ref().unwrap().numbers, numbers);
    }
    #[test]
    fn test_baseline_algo_get_numbers_count() {
        let numbers = generate_default_init_values_for_numbers();
        let ba = BaselineAlgo::new(numbers).unwrap();
        assert_eq!(ba.get_numbers_count(), 5040);
        let numbers = vec![String::from("1234"), String::from("5678")];
        let ba = BaselineAlgo::new(numbers).unwrap();
        assert_eq!(ba.get_numbers_count(), 2);
    }
    #[test]
    fn test_basic_baseline_algo_guess() {
        let numbers = generate_default_init_values_for_numbers();
        let mut ba = super::BaselineAlgo::new(numbers).unwrap();
        let guess = ba.guess();
        assert_eq!(guess.as_ref().is_some(), true);
        assert_eq!(ba.numbers.len(), 5039);
        assert_eq!(ba.past_guesses.len(), 1);
        assert_eq!(ba.last_guess_updated, false);
        assert_eq!(ba.past_guesses[0].0, guess.unwrap());
        assert_eq!(ba.past_guesses[0].1, (usize::MAX, usize::MAX));
        
        let guess = ba.guess();
        assert_eq!(guess.as_ref().is_some(), false);

        let numbers = vec![String::from("1234"), String::from("5678")];
        let mut ba = super::BaselineAlgo::new(numbers).unwrap();
        let guess = ba.guess();
        assert_eq!(guess.as_ref().is_some(), true);
        assert_eq!(ba.numbers.len(), 1);
        assert_eq!(ba.past_guesses.len(), 1);
        assert_eq!(ba.last_guess_updated, false);
        assert_eq!(ba.past_guesses[0].0, guess.unwrap());
        assert_eq!(ba.past_guesses[0].1, (usize::MAX, usize::MAX));
    }
    #[test]
    fn test_find_valid_numbers(){
        let numbers = vec![String::from("1234"), String::from("5678"), String::from("8921")];
        let mut ba = BaselineAlgo::new(numbers).unwrap();
        let guess = ba.guess().unwrap();
        let valid_numbers = ba.pub_find_valid_numbers((guess.clone(), (4, 0))).unwrap();
        assert_eq!(valid_numbers.len(), 0);
    }
    #[test]
    fn test_basic_baseline_algo_incorporate_guess_feedback() {
        let numbers = generate_default_init_values_for_numbers();
        let mut ba = BaselineAlgo::new(numbers).unwrap();
        let guess = ba.guess().unwrap();
        let res = ba.incorporate_guess_feedback(4, 0);
        assert_eq!(res.as_ref().is_some(), true);
        assert_eq!(res.unwrap(), true);
        assert_eq!(ba.get_numbers_count(), 0);
        assert_eq!(ba.past_guesses.len(), 1);
        assert_eq!(ba.last_guess_updated, true);
        assert_eq!(ba.past_guesses[0].0, guess);
        assert_eq!(ba.past_guesses[0].1, (4, 0));
        
        let numbers = generate_default_init_values_for_numbers();
        let mut ba = BaselineAlgo::new(numbers).unwrap();
        let guess = ba.guess().unwrap();
        let res = ba.incorporate_guess_feedback(5, 0);
        assert_eq!(res.as_ref().is_some(), false);
    }
    #[test]
    // THIS TEST IS IN PROGRESS
    fn test_baseline_algo_pub_find_valid_numbers() {
        let numbers = generate_default_init_values_for_numbers();
        let mut ba = BaselineAlgo::new(numbers).unwrap();
        let guess = ba.guess().unwrap();
        let feedbacks = vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4),
                                                    (1, 0), (1, 1), (1, 2), (1, 3),
                                                    (2, 0), (2, 1), (2, 2),
                                                    (3, 0),
                                                    (4, 0)];
        for feedback in feedbacks {
            let valid_numbers = ba.pub_find_valid_numbers((guess.clone(), feedback)).unwrap();
            // 4 choose feedback.0 times (6 + feedback.1) choose (4 - feedback.0) times (4 - feedback.0)!
            let expected_len = 
            assert_eq!(valid_numbers.len(), expected_len);
        }
        
        let valid_numbers = ba.pub_find_valid_numbers((guess.clone(), (4, 0))).unwrap();
        assert_eq!(valid_numbers.len(), 0);
        let valid_numbers = ba.pub_find_valid_numbers((guess.clone(), (3, 0))).unwrap();
        assert_eq!(valid_numbers.len(), 24);
        let valid_numbers = ba.pub_find_valid_numbers((guess.clone(), (2, 0))).unwrap();
        assert_eq!(valid_numbers.len(), 276);
        let valid_numbers = ba.pub_find_valid_numbers((guess.clone(), (1, 0))).unwrap();
        assert_eq!(valid_numbers.len(), 2024);
        let valid_numbers = ba.pub_find_valid_numbers((guess.clone(), (0, 0))).unwrap();
        assert_eq!(valid_numbers.len(), 5040);
        let valid_numbers = ba.pub_find_valid_numbers((guess.clone(), (0, 1))).unwrap();
        assert_eq!(valid_numbers.len(), 0);
        let valid_numbers = ba.pub_find_valid_numbers((guess.clone(), (0, 2))).unwrap();
        assert_eq!(valid_numbers.len(), 0);
        let valid_numbers = ba.pub_find_valid_numbers((guess.clone(), (0, 3))).unwrap();
        assert_eq!(valid_numbers.len(), 0);
        let valid_numbers = ba.pub_find_valid_numbers((guess.clone(), (0, 4))).unwrap();
        assert_eq!(valid_numbers.len(), 0);
        let valid_numbers = ba.pub_find_valid_numbers((guess.clone(), (1, 1))).unwrap();
        assert_eq!(valid_numbers.len(), 0);
        let valid_numbers = ba.pub_find_valid_numbers((guess.clone(), (1, 2))).unwrap();
        assert_eq!(valid_numbers.len(), 0);
        let valid_numbers = ba.pub_find_valid_numbers((guess.clone(), (1, 3))).unwrap();
        assert_eq!(valid_numbers.len(), 0);
        let valid_numbers = ba.pub_find_valid_numbers((guess.clone(), (1, 4))).unwrap();
        assert_eq!(valid_numbers.len(), 0);
        let valid_numbers = ba.pub_find_valid_numbers((guess.clone(), (2, 1))).unwrap();
        assert_eq!(valid_numbers.len(), 0
    }
}

// fn main() {
//     let mut defaults : Vec<String> = algos::generate_default_init_values_for_numbers();
//     let mut ba: = algos::BaselineAlgo::new(defaults);
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