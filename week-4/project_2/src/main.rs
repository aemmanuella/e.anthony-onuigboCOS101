use std::io;

fn main() {
    // Get the employee's experience
    let mut input = String::new();
    println!("Is the employee experienced? (yes/no): ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let experience = input.trim().to_lowercase() == "yes";

    // Get the employee's age
    input.clear(); // Clear the input string to reuse it
    println!("Enter the age of the employee: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let age: u32 = input.trim().parse().expect("Please enter a valid age");

    // Determine the incentive based on experience and age
    let incentive = if experience {
        if age >= 40 {
            1_560_000
        } else if age >= 30 {
            1_480_000
        } else if age < 28 {
            1_300_000
        } else {
            0 // Catch-all case, based on provided criteria this won't be used
        }
    } else {
        100_000
    };

    println!("The annual incentive for the employee is: N{}", incentive);
}