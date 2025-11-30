use rand::Rng;
use std::io;

fn main() {
    //Initialize
    let mut playing = true;
    let mut guesses: u8 = 0;

    //make random number
    let mut rng = rand::rng();
    let rand_num: u8 = rng.random_range(1..100);

    //print welcome and first guess
    println!("Hilo Guessing Game!\n");
    println!("I'll make a random number from 1-100 and you make a guess.");
    while playing {
        println!("Enter your guess: ");

        //Input
        let mut inpt: String = String::new();
        io::stdin().read_line(&mut inpt).expect("err");
        let guess: u8 = inpt.trim().parse().expect("Not a u8");

        guesses += 1;
        if guess > rand_num {
            println!("Too high!");
        } else if guess < rand_num {
            println!("Too low!");
        } else if guess == rand_num {
            println!("You guessed correct!");
            playing = false;
        }
    }
    println!("You guessed {} time(s).", guesses);
}
