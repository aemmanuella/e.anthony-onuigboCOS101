use std::io::Write;
use std::io;

fn main() {
    //make vectors
    let mut name : Vec<String> = Vec::new();

    let mut matric : Vec<String> = Vec::new();

    let mut department : Vec<String> = Vec::new();

    let mut level : Vec<String> = Vec::new(); 

    loop {
        let mut input1 = String::new();
        println!("Enter your name (or type 'done' to stop) : ");
        io::stdin().read_line(&mut input1).expect("Failed to read line.");
        input1 = input1.trim().to_string();
        if input1 == "done" {
            break;
        }
        name.push(input1);

        let mut input2 = String::new();
        println!("\nEnter your matric. number: ");
        io::stdin().read_line(&mut input2).expect("Failed to read line.");
        input2 = input2.trim().to_string();
        matric.push(input2);

        let mut input3 = String::new();
        println!("\nEnter your department: ");
        io::stdin().read_line(&mut input3).expect("Failed to read line.");
        input3 = input3.trim().to_string();
        department.push(input3);

        let mut input4 = String::new();
        println!("\nEnter your level: ");
        io::stdin().read_line(&mut input4).expect("Failed to read line.");
        input4 = input4.trim().to_string();
        level.push(input4);

        println!("\nStudent data collected!\n");


    }

    println!("\nCollected Student Details:");
    for (i, name) in name.iter().enumerate() {
        println!(
            "Name: {}, Matric Number: {}, Department: {}, Level: {}",
            name, matric[i], department[i], level[i]
        );
    }

    let mut file = std::fs::File::create("SIMS.csv").expect("create failed");
    file.write_all("Student Name, Matric. Number, Department, Level\n".as_bytes()).expect("failed to write header.");
    for (i, name) in name.iter().enumerate() {
        let line = format!("{}, {}, {}, {} \n", name, matric[i], department[i], level[i]);
        file.write_all(line.as_bytes()).expect("failed to write line.");
    }

    println!("Details saved to SIMS.csv");
}
