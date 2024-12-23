use std::fs::OpenOptions;
use std::io;
use std::io::Write;

fn main() {
    
    //ask for type of drink
    let mut drink = String::new();
    println!("Enter the type of drink (Lager, Stout or Non-Alcoholic):");
    io::stdin().read_line(&mut drink).expect("Failed to read line.");

    //ask for name of drink
    let mut name = String::new();
    println!("\nEnter the name of the drink:");
    io::stdin().read_line(&mut name).expect("Failed to read line."); 
    
    //create files
    std::fs::File::create("Lager.txt").expect("create failed");
    std::fs::File::create("Stout.txt").expect("create failed");
    std::fs::File::create("Non-Alcoholic.txt").expect("create failed");


    if drink.trim() == "Lager" {
        let mut file1 = OpenOptions::new().append(true).open("Lager.txt").expect("Failed to open Lager.txt");
        file1.write_all(name.as_bytes()).expect("Failed to write to Lager.txt");
        println!("\nData written to Lager.txt.");
    } else if drink.trim() == "Stout" {
        let mut file2 = OpenOptions::new().append(true).open("Stout.txt").expect("Failed to open Stout.txt");
        file2.write_all(name.as_bytes()).expect("Failed to write to Stout.txt");
        println!("\nData written to Stout.txt.");
    } else if drink.trim() == "Non-Alcoholic" {
        let mut file3 = OpenOptions::new().append(true).open("Non-Alcoholic.txt").expect("Failed to open Non-Alcoholic.txt");
        file3.write_all(name.as_bytes()).expect("Failed to write to Non-Alcoholic.txt");
        println!("\nData written to Non-Alcoholic.txt.");
    } else {
        println!("\nSorry, Invalid input.");
    }
}
