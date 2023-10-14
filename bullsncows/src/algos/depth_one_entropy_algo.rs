use std::collections::HashMap;

// use std::collections::HashSet;
use crate::algos::algos_utils;
// use crate::algos::algos_utils::{get_bulls_and_cows, generate_possible_feedbacks};

pub struct DepthOneEntropyAlgo {
    numbers : Vec<String>,
    possible_feedbacks : Vec<(usize, usize)>,
    past_guesses : Vec<(String, (usize, usize))>,
    last_guess_updated : bool,
    ndigits : usize,
}

impl DepthOneEntropyAlgo {
    pub fn new(numbers: Vec<String>) -> Option<DepthOneEntropyAlgo> {
        if numbers.len() == 0 {
            return None;
        }
        let past_guesses: Vec<(String, (usize, usize))> = Vec::new();
        let last_guess_updated: bool = true;
        let ndigits: usize = numbers[0].len();
        let possible_feedbacks: Vec<(usize, usize)> = algos_utils::generate_possible_feedbacks(ndigits);
        return Some(DepthOneEntropyAlgo {numbers, possible_feedbacks, past_guesses, last_guess_updated, ndigits}); 
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
        let mut best_entropy: f64 = f64::MIN;
        for number in self.numbers.iter() {
            let entropy: f64 = self.calculate_guess_entropy(number.clone()).unwrap();
            if entropy > best_entropy {
                best_entropy = entropy.clone();
                best_guess = number.clone();
            }
        }

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


    fn calculate_guess_entropy(&self, guess: String) -> Option<f64> {
        if self.get_numbers_count() == 1 {
            return None;
        }
        let numbers_but_guess_count: f64 = (self.get_numbers_count() - 1) as f64;
        let mut entropy: f64 = 0.0;
        let mut valid_numbers_count_per_feedback: HashMap<(usize, usize), usize> = HashMap::new();
        for number in self.numbers.iter() {
            let bnc:(usize, usize) = algos_utils::get_bulls_and_cows(number.clone(), guess.clone()).unwrap();
            valid_numbers_count_per_feedback.entry(bnc).and_modify(|e| *e += 1).or_insert(1);
        }
        valid_numbers_count_per_feedback.remove_entry(&(self.ndigits.clone(),0));

        for count in valid_numbers_count_per_feedback.values() {
            let probability: f64 = *count as f64 / numbers_but_guess_count;
            entropy += -probability * probability.log2();
        }
        
        return Some(entropy);
    }

}

mod tests {
    use crate::algos::algos_utils::generate_default_init_values_for_numbers;
    use super::DepthOneEntropyAlgo;
    
    #[test]
    fn test_DepthOneEntropyAlgo_new() {
        let numbers = generate_default_init_values_for_numbers();
        let doea = DepthOneEntropyAlgo::new(numbers);
        assert_eq!(doea.as_ref().is_some(), true);
        assert!(doea.as_ref().unwrap().numbers.len() > 0);
        assert!(doea.as_ref().unwrap().possible_feedbacks.len() > 0);
        assert_eq!(doea.as_ref().unwrap().past_guesses.len(), 0);
        assert_eq!(doea.as_ref().unwrap().last_guess_updated, true);
        assert_eq!(doea.as_ref().unwrap().ndigits, 4);
        let numbers = generate_default_init_values_for_numbers();
        assert_eq!(doea.as_ref().unwrap().numbers, numbers);
    }
    #[test]
    fn test_DepthOneEntropyAlgo_get_numbers_count() {
        let numbers = generate_default_init_values_for_numbers();
        let doea = DepthOneEntropyAlgo::new(numbers).unwrap();
        assert_eq!(doea.get_numbers_count(), 5040);
        let numbers = vec![String::from("1234"), String::from("5678")];
        let doea = DepthOneEntropyAlgo::new(numbers).unwrap();
        assert_eq!(doea.get_numbers_count(), 2);
    }
    #[test]
    fn test_basic_DepthOneEntropyAlgo_guess0() {
        let numbers = generate_default_init_values_for_numbers();
        let mut doea = super::DepthOneEntropyAlgo::new(numbers).unwrap();
        let guess = doea.guess();
        assert_eq!(guess.as_ref().is_some(), true);
        assert_eq!(doea.numbers.len(), 5039);
        assert_eq!(doea.past_guesses.len(), 1);
        assert_eq!(doea.last_guess_updated, false);
        assert_eq!(doea.past_guesses[0].0, *guess.as_ref().unwrap());
        assert_eq!(doea.past_guesses[0].1, (usize::MAX, usize::MAX));
        
        let guess = doea.guess();
        assert_eq!(guess.as_ref().is_some(), false);
    }
    #[test]
    fn test_basic_DepthOneEntropyAlgo_guess1() {
        let numbers = vec![String::from("1234"), String::from("5678")];
        let mut doea = super::DepthOneEntropyAlgo::new(numbers).unwrap();
        let guess = doea.guess();
        assert_eq!(guess.as_ref().is_some(), true);
        assert_eq!(doea.numbers.len(), 1);
        assert_eq!(doea.past_guesses.len(), 1);
        assert_eq!(doea.last_guess_updated, false);
        assert_eq!(doea.past_guesses[0].0, *guess.as_ref().unwrap());
        assert_eq!(doea.past_guesses[0].1, (usize::MAX, usize::MAX));
        
        let numbers = vec![String::from("1234"), String::from("1256"), String::from("7325"), String::from("2091"), String::from("9012"), String::from("1324"), String::from("7891")];
        let mut doea = super::DepthOneEntropyAlgo::new(numbers).unwrap();
        let guess = doea.guess();
        assert_eq!(vec![String::from("1234"), String::from("1256"), String::from("1324")].contains(guess.as_ref().unwrap()), true);
        assert_eq!(guess.as_ref().unwrap(), &String::from("1324"));
    }
    #[test]
    fn test_basic_DepthOneEntropyAlgo_guess2() {
        let numbers = vec![String::from("1234"), String::from("1256"), String::from("7325"), String::from("2091"), String::from("9012"), String::from("1324"), String::from("7891")];
        let mut doea = super::DepthOneEntropyAlgo::new(numbers).unwrap();
        let guess = doea.guess();
        assert_eq!(vec![String::from("1234"), String::from("1256"), String::from("1324")].contains(guess.as_ref().unwrap()), true);
        assert_eq!(guess.as_ref().unwrap(), &String::from("1324"));
    }
    // #[test]
    // fn test_basic_DepthOneEntropyAlgo_incorporate_guess_feedback() {
    //     let numbers = generate_default_init_values_for_numbers();
    //     let mut doea = DepthOneEntropyAlgo::new(numbers).unwrap();
    //     let guess = doea.guess().unwrap();
    //     let res = doea.incorporate_guess_feedback(4, 0);
    //     assert_eq!(res.as_ref().is_some(), true);
    //     assert_eq!(res.unwrap(), true);
    //     assert_eq!(doea.get_numbers_count(), 0);
    //     assert_eq!(doea.past_guesses.len(), 1);
    //     assert_eq!(doea.last_guess_updated, true);
    //     assert_eq!(doea.past_guesses[0].0, guess);
    //     assert_eq!(doea.past_guesses[0].1, (4, 0));
        
    //     let numbers = generate_default_init_values_for_numbers();
    //     let mut doea = DepthOneEntropyAlgo::new(numbers).unwrap();
    //     let guess = doea.guess().unwrap();
    //     let res = doea.incorporate_guess_feedback(5, 0);
    //     assert_eq!(res.as_ref().is_some(), false);
    // }
}
