use rand::{seq::IteratorRandom, thread_rng};

pub struct Game {
    guess_count : u64,
    answer : String
}

impl Game {
    pub fn new(length : usize) -> Game {
        let mut rng = thread_rng();
        let sample = (0..10).choose_multiple(&mut rng, length);

        let answer = sample.iter().map(|u| -> char {
                char::from_digit(*u, 10).expect("Gamee error")
            }).collect();

        return Game { guess_count: 0, answer };
    }

    pub fn get_answer(&self) -> &str {
        return self.answer.as_str();
    }

    pub fn get_guess_count(&self) -> u64 {
        return self.guess_count;
    }

    pub fn get_answer_length(&self) -> usize {
        self.answer.len()
    }

    pub fn make_guess(&mut self, guess: &str) -> Option<(usize, usize)> {
        if guess.len() != self.answer.len() || !guess.chars().all(char::is_alphanumeric) {
            return None;
        }

        self.guess_count += 1;
        
        let bulls = guess.chars().zip(self.answer.chars()).filter(|(a, b)| {
            a == b
        }).count();

        let cows = guess.chars().filter(|&c| {
            self.answer.contains(c)
        }).count();

        return Some((bulls, cows - bulls));
    }
}