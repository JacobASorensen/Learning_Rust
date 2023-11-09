use std::io;
use std::io::Write;
/*
I will attempt to implement each of the following programs:

Convert temperatures between Fahrenheit and Celsius.
Generate the nth Fibonacci number.
Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

 */

fn main() {
    let mut do_main_loop = true;

        while do_main_loop {
        println!("\n\nHello, what would you like to do? You have 4 choices:");
        println!("0: Convert between Fahrenheit and Celsius");
        println!("1: Generate the nth Fibonacci number (up to 186th)");
        println!("2: Print lyrics to \"The Twelve Days of Christmas\"");
        println!("3: Exit the Program");
        
        println!("\nPlease input your option; an integer from 0 to 3.");


        let choice: u32 = get_number::<u32>("That was not a valid integer".to_string());

        match choice {
            0 => do_temp_conversion(),
            1 => do_nth_fibonnacci(),
            2 => do_christmas_days(),
            3 => do_main_loop = false,
            4_u32..=u32::MAX => println!("That was not an integer from 0 to 3."),
        }

    }
}

fn do_temp_conversion() {
    
    let mut f_to_c = true;
    let mut has_found_way = false;

        while !has_found_way {
        
        println!("\nSo do you want to convert from Fahrenheit to Celsius or Celsius to Fahrenheit?");
        println!("0: Fahrenheit to Celsius");
        println!("1: Celsius to Fahrenheit");
        println!("\nPlease input your option; an integer from 0 to 1.");
        
        let choice: u32 = get_number::<u32>("That was not a valid integer".to_string());

        match choice {
            0 => {f_to_c = true;  has_found_way = true;},
            1 => {f_to_c = false; has_found_way = true;},
            2_u32..=u32::MAX => println!("That was not an integer from 0 to 1."),
        };
    }

    let mut temperature_name;
    if f_to_c {
        temperature_name = "Fahrenheit";
    } else {
        temperature_name = "Celsius";
    }

    println!("Enter a {temperature_name} temperature: ");

    let temperature = get_number::<f32>("That was not a valid number".to_string());

    let new_temperature;

    if f_to_c {
        new_temperature = (temperature - 32.0)/1.8;
        temperature_name = "Celsius";
    } else {
        new_temperature = temperature*1.8 + 32.0;
        temperature_name = "Fahrenheit";
    }
    
    println!("Your temperature in {temperature_name} is: {new_temperature}");
    please_press_enter();
}

fn do_nth_fibonnacci() {

    println!("\nPlease input the Fibonacci number that you want");

    let fibonacci_index = get_number::<usize>("That was not a valid integer".to_string());
    
    let fibonacci_num;
    if fibonacci_index <= 32 {
        fibonacci_num = closed_form_fibonacci(fibonacci_index) as u128;
    } else {
        let mut fib_numbers = Vec::new();
        let mut current_fib_num = 33;
        for i in 30..32 {
            fib_numbers.push(closed_form_fibonacci(i) as u128);
        }
        fib_numbers.push(fib_numbers[fib_numbers.len() - 1] + fib_numbers[fib_numbers.len() - 2]);
        
        while current_fib_num <= fibonacci_index {
            fib_numbers[current_fib_num % 3] = fib_numbers[(current_fib_num + 1) % 3] + fib_numbers[(current_fib_num + 2) % 3];
            current_fib_num += 1;
        }
        //println!("n % 3: {} n+1%3: {} n+2%3: {} ", fib_numbers[0],fib_numbers[1],fib_numbers[2]);
        fibonacci_num = fib_numbers[(current_fib_num - 1) % 3];
    }
    
    println!("Fibonacci number {fibonacci_index} is {fibonacci_num}");
    please_press_enter();
}

// I primarily kept this function in because I found it interesting
// I originally wrote this around the time I was taking discrete II
// This is using Binet's formula
// This implementation is only accurate up to the 32nd fibonacci number.
// It could likely be improved by using higher definition floats
fn closed_form_fibonacci(fibonacci_index: usize) -> u32{
    let square_root_of_five = f32::powf(5.0, 0.5);
    let inverse_sqrt_of_five = 1.0/square_root_of_five;

    let mut left_num = 0.5*(1.0 + square_root_of_five);
    let mut right_num = 0.5*(1.0 - square_root_of_five);

    left_num = f32::powf(left_num,fibonacci_index as f32);
    right_num = f32::powf(right_num,fibonacci_index as f32);

    let mut fibonacci_num = left_num - right_num;
    fibonacci_num = inverse_sqrt_of_five*fibonacci_num;
    
    let fibonacci_num = fibonacci_num as u32;
    return fibonacci_num;
}

fn do_christmas_days() {
    let days = vec!["first","second","third","fourth","fifth","sixth",
                    "seventh","eighth","ninth","tenth","eleventh","twelfth"];
    let gifts = vec!["And a partridge in a pear tree",
                    "Two turtle doves",
                    "Three french hens",
                    "Four calling birds",
                    "FIIIVVVEEEE GOLLLDDEEENNNN RRIIIIIINNNGGGSSSS",
                    "Six geese a-laying",
                    "Seven swans a-swimming",
                    "Eight maids a-milking",
                    "Nine ladies dancing",
                    "Ten lords a-leaping",
                    "Eleven pipers piping",
                    "Twelve drummers drumming"];

    let mut current_day = 0_usize;
    for day in &days {
        println!("On the {day} day of Christmas my true love gave to me:\n");
        if day == &days[0] {
            println!("A {}",&gifts[0][6..]);
        } else {
            for i in 0_usize..=current_day {
                println!("{}",&gifts[current_day - i]);
            }
        }
        current_day += 1;

        please_press_enter();

    }
}
/*
fn find_prime_factorizations() {
    
const FIRST_THOUSAND_PRIMES: [i32; 168] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 
53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109,
113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 
181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 
251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 
317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 
397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 
463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 
557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 
619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 
701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 
787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 
863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 
953, 967, 971, 977, 983, 991, 997];
}
*/

fn please_press_enter() {
    let mut choice = String::new();
    print!("...");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
}

fn get_number<A: std::str::FromStr>(error_message: String) -> A{
    loop{
            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");

            let choice: A = match choice.trim().parse()  {
                Ok(num) => num,
                Err(_) => {
                    println!("{}",error_message);
                    continue;
                },
            };
            return choice;
    }
}