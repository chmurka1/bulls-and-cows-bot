mod game;

fn main() {
    let mut g = game::Game::new(4); 
    println!("Guess the number: {}", g.get_answer());
    
    for _ in 1..100 {
        let mut user_guess = String::new();
        std::io::stdin().read_line(&mut user_guess).expect("Error while reading user input");
        user_guess = String::from(user_guess.trim());

        match g.make_guess(&user_guess) {
            None => {
                println!("Invalid input!");
                continue;
            }
            Some((bulls, cows)) => {
                println!("Bulls: {}, cows: {}", bulls, cows);

                if bulls == g.get_answer_length() && cows == 0 {
                    println!("You won after {} guesses", g.get_guess_count());
                }
            }
        }
    }
}
