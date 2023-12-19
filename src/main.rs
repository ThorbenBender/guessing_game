use rand::Rng;

fn main() {
    println!("Welcome to the guessing game!!!");
    println!("You have to guess a random number between 0-100. Good luck!");

    let random_number: u8 = rand::thread_rng().gen_range(0..=100);

    let mut input = String::new();

    loop {
        println!("Try to the guess the number!");
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => {
                println!("Coulndt read your input!!!");
                break;
            }
        }

        match dbg!(input.trim()).parse::<u8>() {
            Ok(guess) => {
                if guess < random_number {
                    println!("The number is too small!!");
                    input.clear();
                } else if guess > random_number {
                    println!("The number is too big!!");
                    input.clear();
                } else {
                    println!("You guessed the number!!!");
                    break;
                }
            }
            Err(_) => {
                println!("Invalid number!!!");
                continue;
            }
        }
    }
}
