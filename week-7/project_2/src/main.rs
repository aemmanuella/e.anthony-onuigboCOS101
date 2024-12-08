fn main() {
    // Vector of tuples containing developer information: (Name, Years of Experience)
    let developers = vec![
        ("Ada", 5),
        ("John", 10),
        ("Betty", 8),
        ("Emmanuella", 15),
        ("Pope", 12),
    ];

    // Find the developer with the highest years of experience
    let mut top_developer = &developers[0];
    for dev in &developers {
        if dev.1 > top_developer.1 {
            top_developer = dev;
        }
    }

    println!(
        "The developer with the highest years of experience is {} with {} years.",
        top_developer.0, top_developer.1
    );
}
