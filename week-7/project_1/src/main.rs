use std::io;

fn main() {
let roles = vec![
    // APS 1-2
    ("Office Administrator", "Intern", "APS 1-2", 0, 2),
    ("Academic", "-", "APS 1-2", 0, 2),
    ("Lawyer", "Paralegal", "APS 1-2", 0, 2),
    ("Teacher", "Placement", "APS 1-2", 0, 2),

    // APS 3-5
    ("Office Administrator", "Administrator", "APS 3-5", 3, 5),
    ("Academic", "Research Assistant", "APS 3-5", 3, 5),
    ("Lawyer", "Junior Associate", "APS 3-5", 3, 5),
    ("Teacher", "Classroom Teacher", "APS 3-5", 3, 5),

    // APS 5-8
    ("Office Administrator", "Senior Administrator", "APS 5-8", 5, 8),
    ("Academic", "PhD Candidate", "APS 5-8", 5, 8),
    ("Lawyer", "Associate", "APS 5-8", 5, 8),
    ("Teacher", "Snr Teacher", "APS 5-8", 5, 8),

    // EL1 8-10
    ("Office Administrator", "Office Manager", "EL1 8-10", 8, 10),
    ("Academic", "Post-Doc Researcher", "EL1 8-10", 8, 10),
    ("Lawyer", "Senior Associate 1-2", "EL1 8-10", 8, 10),
    ("Teacher", "Leading Teacher", "EL1 8-10", 8, 10),

    // EL2 10-13
    ("Office Administrator", "Director", "EL2 10-13", 10, 13),
    ("Academic", "Senior Lecturer", "EL2 10-13", 10, 13),
    ("Lawyer", "Senior Associate 3-4", "EL2 10-13", 10, 13),
    ("Teacher", "Deputy Principal", "EL2 10-13", 10, 13),

    // SES
    ("Office Administrator", "CEO", "SES", 13, 50),
    ("Academic", "Dean", "SES", 13, 50),
    ("Lawyer", "Partner", "SES", 13, 50),
    ("Teacher", "Principal", "SES", 13, 50),
];


    println!("Welcome to the Public Service APS level checker!");

    println!("Please Enter your profession: \n");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line.");
    let profession = input1.trim();

    println!("Please enter your role: \n");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line.");
    let role = input2.trim();

    println!("Please enter your years of work experience: \n");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read line.");
    let years:i32 = input3.trim().parse().expect("Invalid input");

    if roles[0].0 == profession && roles[0].1 == role && (roles[0].3..=roles[0].4).contains(&years) {
    println!("The APS level is: {}", roles[0].2);
} else if roles[1].0 == profession && roles[1].1 == role && (roles[1].3..=roles[1].4).contains(&years) {
    println!("The APS level is: {}", roles[1].2);
} else if roles[2].0 == profession && roles[2].1 == role && (roles[2].3..=roles[2].4).contains(&years) {
    println!("The APS level is: {}", roles[2].2);
} else if roles[3].0 == profession && roles[3].1 == role && (roles[3].3..=roles[3].4).contains(&years) {
    println!("The APS level is: {}", roles[3].2);
} else if roles[4].0 == profession && roles[4].1 == role && (roles[4].3..=roles[4].4).contains(&years) {
    println!("The APS level is: {}", roles[4].2);
} else if roles[5].0 == profession && roles[5].1 == role && (roles[5].3..=roles[5].4).contains(&years) {
    println!("The APS level is: {}", roles[5].2);
} else if roles[6].0 == profession && roles[6].1 == role && (roles[6].3..=roles[6].4).contains(&years) {
    println!("The APS level is: {}", roles[6].2);
} else if roles[7].0 == profession && roles[7].1 == role && (roles[7].3..=roles[7].4).contains(&years) {
    println!("The APS level is: {}", roles[7].2);
} else if roles[8].0 == profession && roles[8].1 == role && (roles[8].3..=roles[8].4).contains(&years) {
    println!("The APS level is: {}", roles[8].2);
} else if roles[9].0 == profession && roles[9].1 == role && (roles[9].3..=roles[9].4).contains(&years) {
    println!("The APS level is: {}", roles[9].2);
} else if roles[10].0 == profession && roles[10].1 == role && (roles[10].3..=roles[10].4).contains(&years) {
    println!("The APS level is: {}", roles[10].2);
} else if roles[11].0 == profession && roles[11].1 == role && (roles[11].3..=roles[11].4).contains(&years) {
    println!("The APS level is: {}", roles[11].2);
} else if roles[12].0 == profession && roles[12].1 == role && (roles[12].3..=roles[12].4).contains(&years) {
    println!("The APS level is: {}", roles[12].2);
} else if roles[13].0 == profession && roles[13].1 == role && (roles[13].3..=roles[13].4).contains(&years) {
    println!("The APS level is: {}", roles[13].2);
} else if roles[14].0 == profession && roles[14].1 == role && (roles[14].3..=roles[14].4).contains(&years) {
    println!("The APS level is: {}", roles[14].2);
} else if roles[15].0 == profession && roles[15].1 == role && (roles[15].3..=roles[15].4).contains(&years) {
    println!("The APS level is: {}", roles[15].2);
} else if roles[16].0 == profession && roles[16].1 == role && (roles[16].3..=roles[16].4).contains(&years) {
    println!("The APS level is: {}", roles[16].2);
} else if roles[17].0 == profession && roles[17].1 == role && (roles[17].3..=roles[17].4).contains(&years) {
    println!("The APS level is: {}", roles[17].2);
} else if roles[18].0 == profession && roles[18].1 == role && (roles[18].3..=roles[18].4).contains(&years) {
    println!("The APS level is: {}", roles[18].2);
} else if roles[19].0 == profession && roles[19].1 == role && (roles[19].3..=roles[19].4).contains(&years) {
    println!("The APS level is: {}", roles[19].2);
} else if roles[20].0 == profession && roles[20].1 == role && (roles[20].3..=roles[20].4).contains(&years) {
    println!("The APS level is: {}", roles[20].2);
} else if roles[21].0 == profession && roles[21].1 == role && (roles[21].3..=roles[21].4).contains(&years) {
    println!("The APS level is: {}", roles[21].2);
} else if roles[22].0 == profession && roles[22].1 == role && (roles[22].3..=roles[22].4).contains(&years) {
    println!("The APS level is: {}", roles[22].2);
} else if roles[23].0 == profession && roles[23].1 == role && (roles[23].3..=roles[23].4).contains(&years) {
    println!("The APS level is: {}", roles[23].2);
} else {
    println!("No matching APS level found.");
}
 

}
