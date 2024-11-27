use std::io;

fn trapezium(a:f32 , b:f32 , h:f32){

    let formula = h/2.0 * (a + b);
    println!("\nThe Area of Trapezium is {} cm2", formula);
}

fn rhombus(a:f32 , b:f32){
    let formula = 0.5 * a * b;
    println!("\nThe Area of Rhombus is {} cm2", formula);
}

fn parallelogram(a:f32, b:f32){
    let formula =  a * b;
    println!("The Area of Parallelogram is {} cm2", formula);
}

fn cube(a:f32){
    let formula = 6.0 * a.powf(2.0);
    println!("The Area of the Cube is {} cm2", formula);
}

fn cylinder(a:f32, b:f32){
    let formula = 3.142 * a.powf(2.0) * b;
    println!("The Volume of the Cylinder is {} cm3", formula);
}

fn main() {
    println!("\nWelcome to Ella's Calculator");
    println!("\nPlease choose an equation for your Calculation:");
    println!("1. Area of Trapezium formula = height / 2 * (base 1 + base 2)");
    println!("2. Area of Rhombus formula = 1/2 * diagonal 1 * diagonal 2");
    println!("3. Area of parallelogram formula = base * altitude");
    println!("4. Area of Cube formula = 6 * (length of side)^2");
    println!("5. Volume of Cylinder formula = π * radius^2 * height");

    let mut input1 = String::new();
    println!("\nChoose an equation:");
    io::stdin().read_line(&mut input1).expect("Failed to read_line");
    let input1:i32 = input1.trim().parse().expect("Not a valid input.");


    if input1 == 1 {
        let mut a = String::new();
        println!("\nInput parameter for base 1(cm): ");
        io::stdin().read_line(&mut a).expect("Not a valid input");
        let a:f32 = a.trim().parse().expect("Not a valid input");

        let mut b = String::new();
        println!("\nInput parameter for base 2(cm): ");
        io::stdin().read_line(&mut b).expect("Not a valid input");
        let b:f32 = b.trim().parse().expect("Not a valid input");

        let mut h = String::new();
        println!("\nInput parameter for height(cm): ");
        io::stdin().read_line(&mut h).expect("Not a valid input");
        let h:f32 = h.trim().parse().expect("Not a valid input");

        trapezium(a, b, h)
    }
    else if input1 == 2{
        let mut a = String::new();
        println!("\nInput parameter for diagonal 1(cm): ");
        io::stdin().read_line(&mut a).expect("Not a valid input");
        let a:f32 = a.trim().parse().expect("Not a valid input");

        let mut b = String::new();
        println!("\nInput parameter for diagonal 2(cm): ");
        io::stdin().read_line(&mut b).expect("Not a valid input");
        let b:f32 = b.trim().parse().expect("Not a valid input");

        rhombus(a, b);
    }
    else if input1 == 3{
       let mut a = String::new();
        println!("\nInput parameter for base(cm): ");
        io::stdin().read_line(&mut a).expect("Not a valid input");
        let a:f32 = a.trim().parse().expect("Not a valid input");

        let mut b = String::new();
        println!("\nInput parameter for altitude(cm): ");
        io::stdin().read_line(&mut b).expect("Not a valid input");
        let b:f32 = b.trim().parse().expect("Not a valid input");

        parallelogram(a, b);
    }
    else if input1 == 4{
        let mut a = String::new();
        println!("\nInput parameter for length of the side(cm): ");
        io::stdin().read_line(&mut a).expect("Not a valid input");
        let a:f32 = a.trim().parse().expect("Not a valid input");
        cube(a);
    }
    else if input1 == 5{
        let mut a = String::new();
        println!("\nInput parameter for the radius(cm): ");
        io::stdin().read_line(&mut a).expect("Not a valid input");
        let a:f32 = a.trim().parse().expect("Not a valid input");

        let mut b = String::new();
        println!("\nInput parameter for the height(cm): ");
        io::stdin().read_line(&mut b).expect("Not a valid input");
        let b:f32 = b.trim().parse().expect("Not a valid input");

        println!("Using pi as (3.142).");
        cylinder(a, b);
    }
    else {
        println!("Sorry, Invalid input");
    }

    println!("\nThank you for using Ella's Calculator!");
}
