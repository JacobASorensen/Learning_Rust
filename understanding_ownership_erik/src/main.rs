fn main() {
    {
        let mut s = String::from("Hello");
        s.push_str(
            ", world!"
        );
    
        println!("{s}");
    }

    let s1 = String::from("hello"); // Strings are on the heap. Literals are on the stack
    let s2 = s1.clone(); // performs a deep copy of s1 instead of just moving the pointer

    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5; 
    let y = x.clone(); // Both literals are on the stack so a x.clone() doesn't actually do anything 
    // Copy can be used for values on the stack
    // clone can be used for values on the heap

    println!("x = {}, y = {}", x, y);

    let _s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1
    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3

    // References s3 using & which just points to this variable in the stack and doesn't move it
    // This allows us to refer the value of s3 to the function without the function taking ownership of it
    let len = calculate_length(&s3); 
    println!("The length of '{}' is {}.", s3, len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("Mutable reference string is {}", s);

    // We cant make simultaneous mutable references to the same object
    // but we can still create new scopes that access the same reference
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        println!("r1 inside {}", r1)
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
    println!("r2 outside {}", r2);

    // Again we can't make simulateous mutable references to the same object
    // but we can make simulateous immutable references to the same object
    // but again we can't make a mutable reference to an object we already have a immutable reference for 
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    println!("{}, {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3); // but this works because r1 and r2 were released after they were used by println
    

    // Slices!
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    let start = &s[..5];
    let end = &s[6..];
    println!("Hello: {hello}");
    println!("World: {world}");
    println!("start: {start}");
    println!("end: {end}");

    let first = first_word(&s);
    println!("{first}");


    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let _word = first_word(&my_string[0..6]);
    let _word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let _word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let _word = first_word(&my_string_literal[0..6]);
    let _word = first_word(&my_string_literal[..]);
    
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("{word}");


    // Slices work on arrays too
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
    
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

let some_string = String::from("yours"); // some_string comes into scope

some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
             // scope and takes ownership

a_string  // a_string is returned and moves out to the calling function to return ownership
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped. This is called borrowing

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}  

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}