use std::collections::{HashSet, HashMap};

pub fn get_bulls_and_cows(number0: String, number1: String) -> Option<(usize, usize)> {
    if number0.len() != number1.len() {
        return None;
    }
    let bulls: usize = number0.chars().zip(number1.chars()).filter(|(a, b)| {
        a == b
    }).count();
    let cows: usize = number0.chars().filter(|&c| {
        number1.contains(c)
    }).count() - bulls;
    return Some((bulls, cows));
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

pub fn generate_possible_feedbacks(ndigits:usize) -> Vec<(usize, usize)> {
    let mut possible_feedbacks: Vec<(usize, usize)> = Vec::new();
    if ndigits == 1 {
        possible_feedbacks.push((1, 0));
        return possible_feedbacks;
    }
    
    for b in 0..(ndigits-1) {
        for c in 0..(ndigits-b+1) {
            possible_feedbacks.push((b, c));
        }
    }
    possible_feedbacks.push((ndigits-1, 0));
    possible_feedbacks.push((ndigits, 0));
    return possible_feedbacks;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_bulls_and_cows() {
        use super::get_bulls_and_cows;
        assert_eq!(get_bulls_and_cows(String::from("1234"), String::from("1234")), Some((4, 0)));
        assert_eq!(get_bulls_and_cows(String::from("1234"), String::from("4321")), Some((0, 4)));
        assert_eq!(get_bulls_and_cows(String::from("1234"), String::from("5678")), Some((0, 0)));
        assert_eq!(get_bulls_and_cows(String::from("1234"), String::from("1235")), Some((3, 0)));
        assert_eq!(get_bulls_and_cows(String::from("1234"), String::from("1256")), Some((2, 0)));
        assert_eq!(get_bulls_and_cows(String::from("1234"), String::from("1243")), Some((2, 2)));
    }
    #[test]
    fn test_generate_default_init_values_for_numbers_output() {
        use super::generate_default_init_values_for_numbers;
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
    fn test_generate_possible_feedbacks() {
        use super::generate_possible_feedbacks;
        let possible_feedbacks = generate_possible_feedbacks(1);
        assert_eq!(possible_feedbacks.len(), 1);
        let possible_feedbacks = generate_possible_feedbacks(4);
        assert_eq!(possible_feedbacks.len(), 14);
        let possible_feedbacks = generate_possible_feedbacks(5);
        assert_eq!(possible_feedbacks.len(), 6*7/2-1);
    }
}