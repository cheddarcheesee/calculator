use std::io;
/*
simple calculator programmed in Rust :P


mostly made by me (cheese) but @Arne6764 helped out too.

sorry for low quality, i made this during math and i'm still learning Rust. enjoy the calculator :D

*/

// main function
fn solve_math() {
    let mut number1 = String::new();
    let mut number2 = String::new();
    let mut operation = String::new();
    // ^
    // getting inputs and storing them in variables
    // v
    println!("Enter your first number: ");
    io::stdin().read_line(&mut number1).expect("Failed to read line.");
    let number1_int: i32 = number1.trim().parse().unwrap(); 
    // converting the input to integer 
    println!("Enter your operatation: ");
    io::stdin().read_line(&mut operation).expect("Failed to read line.");
    operation = operation.trim().to_string();

    println!("Enter your second number: ");
    io::stdin().read_line(&mut number2).expect("Failed to read line.");
    // converting again
    let number2_int: i32 = number2.trim().parse().unwrap(); 
    // calculation and if statement
    if operation == "+" {
      println!("{}", number1_int + number2_int);
      solve_math();
    }
    else if operation == "-" {
      println!("{}", number1_int - number2_int);
      solve_math(); // calling the function again to re-use calculator
    }
    else if operation == "*" {
      println!("{}", number1_int * number2_int);
      solve_math(); // multiplication
    }
     else if operation == "/" {
       println!("{}", number1_int / number2_int);
       solve_math();
     }
     else if operation == "q" {
       println!("Goodbye!");
       // adding this statement to exit the program and prevent the recursion error :/
     } 
     else {
       println!("Try again.");
       solve_math();
     } // this path is for people who dont understand the assignment
    
}

// you know the drill
fn main() {
    println!("welcome to the calculator (i made this in my math class when i should be doing work)");
    solve_meth();
}
