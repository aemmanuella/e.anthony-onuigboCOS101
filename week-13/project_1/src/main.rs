use std::{fs, io};

fn main() {
    let role = get_user_role();
    match role.as_str() {
        "admin" => display_sql_file("globacom_dbase.sql"),
        "project manager" => display_sql_file("project_tb.sql"),
        "employee" => display_sql_file("staff_tb.sql"),
        "customer" => display_sql_file("customer_tb.sql"),
        "vendor" => display_sql_file("dataplan_tb.sql"),
        _ => println!("Invalid role entered."),
    }
}

fn get_user_role() -> String {
    println!("Enter your role (admin, project manager, employee, customer, vendor):");
    let mut role = String::new();
    io::stdin().read_line(&mut role).expect("Failed to read input");
    role.trim().to_lowercase()
}

fn display_sql_file(file_path: &str) {
    let contents = fs::read_to_string(file_path);

    if contents.is_ok() {
        println!("{}", contents.unwrap());
    } else {
        println!("Failed to read {}", file_path);
    }
}
