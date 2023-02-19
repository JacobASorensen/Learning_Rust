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
        println!("1: Generate the nth Fibonacci number");
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
    println!("\nSo do you want to convert from Fahrenheit to Celsius or Celsius to Fahrenheit?");
    println!("0: Fahrenheit to Celsius");
    println!("1: Celsius to Fahrenheit");
    println!("\nPlease input your option; an integer from 0 to 1.");
    
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

    let square_root_of_five = f32::powf(5.0, 0.5);
    let inverse_sqrt_of_five = 1.0/square_root_of_five;

    println!("\nPlease input the Fibonacci number that you want");

    let fibonacci_index = get_number::<u32>("That was not a valid integer".to_string());

    let mut left_num = 0.5*(1.0 + square_root_of_five);
    let mut right_num = 0.5*(1.0 - square_root_of_five);

    left_num = f32::powf(left_num,fibonacci_index as f32);
    right_num = f32::powf(right_num,fibonacci_index as f32);

    let mut fibonacci_num = left_num - right_num;
    fibonacci_num = inverse_sqrt_of_five*fibonacci_num;
    
    let fibonacci_num = fibonacci_num as u32;
    println!("The {fibonacci_index}th fibonacci number is {fibonacci_num}");
    please_press_enter();
}

fn do_christmas_days() {
    let days = vec!["first","second","third","fourth","fifth","sixth",
                    "seventh","eighth","ninth","tenth","eleventh","twelfth"];
    let gifts = vec!["A partridge in a pear tree",
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
        for i in 0_usize..=current_day {
            println!("{}",gifts[i]);
        }
        current_day += 1;

        please_press_enter();

    }
}

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