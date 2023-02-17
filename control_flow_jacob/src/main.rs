use std::io;

/*
I will attempt to implement each of the following programs:

Convert temperatures between Fahrenheit and Celsius.
Generate the nth Fibonacci number.
Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

 */

fn main() {
    let mut doMainLoop = true;

        while doMainLoop {
        println!("\n\nHello, what would you like to do? You have 4 choices:");
        println!("0: Convert between Fahrenheit and Celsius");
        println!("1: Generate the nth Fibonacci number");
        println!("2: Print lyrics to \"The Twelve Days of Christmas\"");
        println!("3: Exit the Program");

        let mut choice = String::new();
        
        println!("\nPlease input your option; an integer from 0 to 3.");
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse()  {
            Ok(num) => num,
            Err(_) => {
                println!("That was not an integer from 0 to 3.");
                continue;
            },
        };

        match choice {
            0 => do_temp_conversion(),
            1 => do_nth_fibonnacci(),
            2 => do_christmas_days(),
            3 => doMainLoop = false,
            4_u32..=u32::MAX => println!("That was not an integer from 0 to 3."),
        }

    }
}

fn do_temp_conversion() {
    println!("\nSo do you want to convert from Fahrenheit to Celsius or Celsius to Fahrenheit?");
    println!("0: Fahrenheit to Celsius");
    println!("1: Celsius to Fahrenheit");
    println!("\nPlease input your option; an integer from 0 to 1.");
    
    let mut FtoC = true;
    let mut hasFoundWay = false;

        while !hasFoundWay {
        
        println!("\nSo do you want to convert from Fahrenheit to Celsius or Celsius to Fahrenheit?");
        println!("0: Fahrenheit to Celsius");
        println!("1: Celsius to Fahrenheit");
        println!("\nPlease input your option; an integer from 0 to 1.");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse()  {
            Ok(num) => num,
            Err(_) => {
                println!("That was not an integer from 0 to 1.");
                continue;
            },
        };

        match choice {
            0 => {FtoC = true;  hasFoundWay = true;},
            1 => {FtoC = false; hasFoundWay = true;},
            2_u32..=u32::MAX => println!("That was not an integer from 0 to 1."),
        };
    }

    let mut tempName = "";
    if FtoC {
        tempName = "Fahrenheit";
    } else {
        tempName = "Celsius";
    }

    println!("Enter a {tempName} temperature: ");

    let mut temperature = 0_f32;

    loop {
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: f32 = match input.trim().parse()  {
            Ok(num) => num,
            Err(_) => {
                println!("That was not a valid number");
                continue;
            },
        };
        temperature = input;
        break;
    };

    let mut newTemperature = 0_f32;

    if FtoC {
        newTemperature = (temperature - 32.0)/1.8;
        tempName = "Celsius";
    } else {
        newTemperature = temperature*1.8 + 32.0;
        tempName = "Fahrenheit";
    }

    
    println!("Your temperature in {tempName} is: {newTemperature}");
}

fn do_nth_fibonnacci() {
    
    let SQUARE_ROOT_OF_FIVE = f32::powf(5.0, 0.5);
    let INVERSE_SQRT_FIVE = 1.0/SQUARE_ROOT_OF_FIVE;

    println!("\nSo do you want to convert from Fahrenheit to Celsius or Celsius to Fahrenheit?");
    println!("\nPlease input the Fibonacci number that you want");

    let mut fibonacciIndex = 0_u32;
    let mut valid = false;

    while !valid {
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse()  {
            Ok(num) => num,
            Err(_) => {
                println!("That was not an integer from 0 to 1.");
                continue;
            },
        };
        valid = true;
        fibonacciIndex = choice;
    }

    let mut leftNum = 0.5*(1.0 + SQUARE_ROOT_OF_FIVE);
    let mut rightNum = 0.5*(1.0 - SQUARE_ROOT_OF_FIVE);

    leftNum = f32::powf(leftNum,fibonacciIndex as f32);
    rightNum = f32::powf(rightNum,fibonacciIndex as f32);

    let mut fibonacciNum = leftNum - rightNum;
    fibonacciNum = INVERSE_SQRT_FIVE*fibonacciNum;
    
    let fibonacciNum = fibonacciNum as u32;
    println!("The {fibonacciIndex}th fibonacci number is {fibonacciNum}");

}

fn do_christmas_days() {

}