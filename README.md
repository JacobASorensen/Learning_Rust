# Learning Rust
## Intro
This repository is where Jacob and Erik Sorensen are learning and practicing their Rust skills. For the most part they are stepping through the [Official Rust Guide Book](https://doc.rust-lang.org/book/ch00-00-introduction.html) chapter by chapter, with creative mini-projects and additional tests sprinkled in there.

Jacob coming from C++ and Erik coming from python have different new things to learn with rust.

The convention for naming project folders is "*subject*_*author*" in snake case format to match Rust's convention. 

To run any of these projects:
1.  make sure you [have Rust and cargo installed](https://doc.rust-lang.org/book/ch01-01-installation.html) 
2. Clone the repository onto your machine
3. navigate to a project folder
4. run the terminal command: ```Cargo run```

## Rust Projects

### Minecraft Chicken Growth Simulator
[***Jacob's Project***](https://github.com/JacobASorensen/Learning_Rust/tree/main/minecraft_chicken_growth_simulator)

This project is meant to simulate the growth rate of Minecraft Chickens according to their in-game mechanics. 

It takes an input number and a target number of chickens, and then runs until the target number of adult chickens is reached.

Jacob made this program in order to generate data so that he could later create a closed-form model of Minecraft chicken population growth.

Example:
![](/images/chicken_example.PNG)

### Understanding Ownership
[***Erik's Project***](https://github.com/JacobASorensen/Learning_Rust/tree/main/understanding_ownership_erik)

Since ownership in Rust is significantly different from Python's garbage collector, Erik used this project to practice and get familiar with Rust's ownership system.

### RPG Structs Practice
[***Jacob's Project***](https://github.com/JacobASorensen/Learning_Rust/tree/main/rpg_structs_practice_jacob)

This project was made to practice using the Serde library's serialize and deserialize methods. It was also good struct practice.

Jacob wanted to build off of the basic console UI work he did previously in the Control Flow project.

The data folder holds the serialized character data. The edit and fight functions are not yet implemented. 

### Variables
[***Erik's Project***](https://github.com/JacobASorensen/Learning_Rust/tree/main/variables_erik)

Since Rust is statically typed, and Python is dynamically typed, Erik used this project to practice instantiating variables of various types.

### Control Flow
[Rust Book Chapter 3](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)

***[Jacob's Project](https://github.com/JacobASorensen/Learning_Rust/tree/main/control_flow_jacob)***

Jacob's version of this project implements all three program ideas listed at the end of chapter 3.5, those being a fibonacci number generator, a Fahrenheit/Celsius temperature convertor, and a program that outputs the lyrics to the twelve days of Christmas. 

For the Fibonacci generator Jacob actually implemented two functions for this, one using Binet's formula up to fibonacci number 32, and then the other one being the standard implementation with O(n) time and O(1) space complexity which works up to fibonacci number 186.

There is not much to say about the other two programs, other than they were fun programming exercises to practice using Rust with.

***[Erik's Project](https://github.com/JacobASorensen/Learning_Rust/tree/main/control_flow_erik)***

Erik practiced Rust's control flow and the syntax associated with it. The elements he practiced were:
- If statements
- The different types of loops
- Loop labels
- Using slices in loops
- Function declarations and uses

### Guessing Game
[Rust book Chapter 2](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

***[Jacob's Project](https://github.com/JacobASorensen/Learning_Rust/tree/main/guessing_game_Jacob)***

Jacob practiced using match statements and revelled in the C-style commenting that Rust uses.

***[Erik's Project](https://github.com/JacobASorensen/Learning_Rust/tree/main/guessing_game_erik)***

Erik also practiced using match statements, although he did not revel in Rust's C-style commenting. 

### Hello Rust
[Rust book Chapter 1](https://doc.rust-lang.org/book/ch01-00-getting-started.html)

***[Jacob's Project](https://github.com/JacobASorensen/Learning_Rust/tree/main/hello-rust-jacob)***

Jacob says "Hello fellow Rustaceans!"

***[Erik's Project](https://github.com/JacobASorensen/Learning_Rust/tree/main/hello-rust-erik)***

Erik says "Hello World!"