use rand::Rng;

enum Result {
    Small,
    Big,
    Correct,
}

fn evaluate_guess(random_number: &u8, guess: &u8) -> Result {
    if random_number > guess {
        return Result::Small;
    } else if random_number < guess {
        return Result::Big;
    } else {
        return Result::Correct;
    }
}

fn change_opponent_guess(
    guess: &u8,
    result: &Result,
    opponent_min_range: &mut u8,
    opponent_max_range: &mut u8,
) {
    match result {
        Result::Small => {
            *opponent_min_range = *guess;
        }
        Result::Big => {
            *opponent_max_range = *guess;
        }
        Result::Correct => {}
    }
}

fn main() {
    println!("Welcome to the guessing game!!!");
    println!("You have to guess a random number between 0-100. Good luck!");

    let random_number: u8 = rand::thread_rng().gen_range(0..=100);

    let mut input = String::new();
    let mut opponent_max_range = 100;
    let mut opponent_min_range = 0;

    loop {
        let opponent_guess = rand::thread_rng().gen_range(opponent_min_range..=opponent_max_range);
        println!("Try to the guess the number!");
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => {
                println!("Coulndt read your input!!!");
                break;
            }
        }

        let guess = match input.trim().parse::<u8>() {
            Ok(num) => num,
            Err(e) => {
                println!("Please input a correct number!!!");
                continue;
            }
        };

        let opponent_result = evaluate_guess(&random_number, &opponent_guess);
        let user_result = evaluate_guess(&random_number, &guess);

        match (&opponent_result, &user_result) {
            (Result::Correct, Result::Correct) => {
                println!("Its a tie. You both guessed right!!!");
                break;
            }
            (_, Result::Correct) => {
                println!("You won!!! God job!");
                break;
            }
            (Result::Correct, _) => {
                println!("You lost!! The opponent guessed it before you did!!!");
                println!("The number was {}", random_number);
                break;
            }
            (_, Result::Big) => {
                println!("Your guess is too big!!! Try again!!!");
                input.clear();
            }
            (_, Result::Small) => {
                println!("Your guess is too small!!! Try again!!!");
                input.clear();
            }
        }

        change_opponent_guess(
            &opponent_guess,
            &opponent_result,
            &mut opponent_min_range,
            &mut opponent_max_range,
        )
    }
}
