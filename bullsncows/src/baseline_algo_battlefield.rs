use crate::baseline_algo::BaselineAlgo;

mod game;
mod baseline_algo;

fn main() {
    let mut g: game::Game = game::Game::new(4);
    let mut defaults : Vec<String> = baseline_algo::generate_default_init_values_for_numbers();
    let mut b:BaselineAlgo = baseline_algo::BaselineAlgo::new(defaults); 
    let mut mode_choice: String = String::new();

    println!("Welcome to the game of Bulls and Cows! Choose mode by typing the prefered option: auto/manual");
    std::io::stdin().read_line(&mut mode_choice).expect("Error while reading user input");
    mode_choice = String::from(mode_choice.trim());
    if mode_choice == "auto" {
        for _ in 1..100 {
            let guess = b.guess();
            println!("No of rem. nums {} |  guess of the BaselineAlgo is: {} ", b.get_numbers().len(), guess);
            match g.make_guess(&guess) {
                None => {
                    println!("Invalid input!");
                    continue;
                }
                Some((bulls, cows)) => {
                    println!("Bulls: {}, cows: {}", bulls, cows);
                    b.add_guess(guess.to_string(), bulls, cows);
                    b.update_numbers();

                    if bulls == g.get_answer_length() && cows == 0 {
                        println!("You won after {} guesses", g.get_guess_count());
                        break;
                    }
                }
            }
        }
    }
    else if mode_choice == "manual" {
        println!("Guess the number: {}", g.get_answer());
        
        for _ in 1..100 {
            let mut user_guess = String::new();
            println!("No of rem. nums {} |  guess of the BaselineAlgo is: {} ", b.get_numbers().len(), b.guess());
            println!("Please enter your guess: ");
            std::io::stdin().read_line(&mut user_guess).expect("Error while reading user input");
            user_guess = String::from(user_guess.trim());

            match g.make_guess(&user_guess) {
                None => {
                    println!("Invalid input!");
                    continue;
                }
                Some((bulls, cows)) => {
                    println!("Bulls: {}, cows: {}", bulls, cows);
                    b.add_guess(user_guess.to_string(), bulls, cows);
                    b.update_numbers();

                    if bulls == g.get_answer_length() && cows == 0 {
                        println!("You won after {} guesses", g.get_guess_count());
                        break;
                    }
                }
            }
        }
    }
    else {
        println!("Invalid input!");
    }
}
