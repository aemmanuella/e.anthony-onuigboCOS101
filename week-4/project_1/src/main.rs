use std::io;

fn main() {
    // Get coefficients a, b, and c
    let mut input = String::new();
    
    println!("Enter the value of a: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let a: f64 = input.trim().parse().expect("Please enter a valid number");
    
    input.clear(); // Clear the input string to reuse it
    println!("Enter the value of b: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let b: f64 = input.trim().parse().expect("Please enter a valid number");

    input.clear();
    println!("Enter the value of c: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let c: f64 = input.trim().parse().expect("Please enter a valid number");

    // Calculate the discriminant
    let discriminant = b * b - 4.0 * a * c;

    // Determine the roots based on the discriminant
    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("There are two distinct real roots: root1 = {} and root2 = {}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("There is exactly one real root: root = {}", root);
    } else {
        println!("There are no real roots.");
    }
}