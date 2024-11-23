use std::io;

fn main() {
    let p:i32 = 3200;
    let f:i32 = 3000;
    let a:i32 = 2500;
    let e:i32 = 2000;
    let w:i32 = 2500;

    //display menu
    println!("Menu: ");
    println!("p = Poundo Yam/Edinkaiko Soup -N3,200");
    println!("f = Fried Rice & Chicken -N3,000");
    println!("a = Amala & Ewedu Soup -N2,500");
    println!("e = Eba&Egusi Soup -N2,000");
    println!("w = White Rice & Stew - N2,500");

    //ask for order
    println!("\nPlease enter the food initial you want: ");
    let mut order = String::new();
    io::stdin().read_line(&mut order).expect("Failed to read line");
    let order = order.trim();
    

    //ask for portions
    println!("\nPlease enter the portions of food you would like (1,2,3,...,etc):");
    let mut portions = String::new();
    io::stdin().read_line(&mut portions).expect("Failed to read line");
    let portions:i32 = portions.trim().parse().expect("Failed to read line.");

    let mut price_per_portion:i32 = 0;

    if order == "p" {
        price_per_portion = p;
    }
    else if order == "f" {
        price_per_portion = f;
    }
    else if order == "a" {
        price_per_portion = a;
    }
    else if order == "e" {
        price_per_portion = e;
    }
    else if order == "w" {
        price_per_portion = w;
    }
    else {
        println!("Invalid initial entered!");
    }


    let total_price:i32 = price_per_portion * portions;

    if total_price > 10000 {
        let discounted_price = total_price - (total_price / 20);
        println!("\nYour total price is N{}. \nYour discounted price is N{}.",total_price, discounted_price);
        println!("Enjoy!");
    }
    else{
        println!("\nYour price is N{}.", total_price);
        println!("Enjoy!");
    }

}
