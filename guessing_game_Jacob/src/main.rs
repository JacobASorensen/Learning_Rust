// This is the standard input-output library
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // Variables are immutable (unchangable) by default

    //println!("The secret number is: {secret_number}");

    //this is great!
    /*
        I love C style commenting!
     */ 

    loop{
        let mut guess = String::new();  // adding the "mut" qualifier allows it to change
        println!("Please input your guess.");
        io::stdin()
            .read_line(&mut guess)           // so not only does & make it a reference, we need to qualify "mut"
            .expect("Failed to read line");  //     so that our .read_line method can change our variable
        
        
        // This is a cool feature of rust
        let guess: u32 = match guess.trim().parse()  {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {guess}");

        // Match is very similar to switch statements in c++! (but they seem better,cooler, and easier to implement!)
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }  
    }
    
    
    // Rust also supports putting variables in place like python! very cool!
}