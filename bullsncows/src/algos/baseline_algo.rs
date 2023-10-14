use std::collections::HashSet;
use rand::seq::SliceRandom;
use crate::algos::algos_utils;
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
        if bulls + cows > self.ndigits || (bulls == self.ndigits-1 && cows == 1) {
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
            let bnc: (usize, usize) = algos_utils::get_bulls_and_cows(guess.0.clone(), number.clone()).unwrap();
            if bnc.0 == guess.1.0 && bnc.1 == guess.1.1 {
                valid_numbers.push(number.clone());
            }
        }
    
    return Some(valid_numbers);
    }
}

#[cfg(test)]
mod tests {
    use crate::algos::algos_utils::generate_default_init_values_for_numbers;
    use super::BaselineAlgo;
    
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
        assert_eq!(ba.past_guesses[0].0, *guess.as_ref().unwrap());
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
        assert_eq!(ba.past_guesses[0].0, *guess.as_ref().unwrap());
        assert_eq!(ba.past_guesses[0].1, (usize::MAX, usize::MAX));
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
}

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