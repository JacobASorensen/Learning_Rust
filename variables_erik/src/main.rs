fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("{spaces}");

    let sum: u8 = 5 + 1;
    println!("{sum}");

    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let e = a[0];
    println!("{e}");
    let a = [3; 5];
    let e = a[0];
    println!("{e}");
    let months: [&str;12] = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let e = months[0];
    println!("{e}");

    another_function(five());

    // This is an expression
    let y = {
        let x = 3;
        // expressions do not include ending semicolons. If a semicolon is 
        // added its turned into a statement which returns no value
        x + 1 
    };

    println!("The value of y is: {y}");
    
}

fn five() -> i32 {
    5 // so semicolon as its an expression that we want to return
}

fn another_function(x: i32) {
    println!("Another function. {x}");
}