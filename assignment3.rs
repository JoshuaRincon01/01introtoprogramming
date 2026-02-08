// Guessing Game (simulated input)

fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret: i32 = 54;
    let mut guess_count = 0;

    // simulated guesses (you can change these numbers)
    let guesses: [i32; 10] = [30, 70, 45, 60, 50, 55, 52, 53, 54, 999];

    let mut i = 0;
    loop {
        if i >= 10 {    // limit number of guesses
            println!("No more guesses available.");
            break;
        }

        let guess = guesses[i];
        guess_count = guess_count + 1;

        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Guess #{}: {} → Correct!", guess_count, guess);
            break;
        } else if result == 1 {
            println!("Guess #{}: {} → Too high", guess_count, guess);
        } else {
            println!("Guess #{}: {} → Too low", guess_count, guess);
        }

        i = i + 1;
    }

    println!("Game over. It took {} guesses.", guess_count);
}