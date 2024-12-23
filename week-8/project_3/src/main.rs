use std::io::Write;

fn main() {
    let name = vec!["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let ministry = vec!["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let zone = vec!["South West", "North East", "South South", "South West", "South East"];

    let mut file = std::fs::File::create("Convicted_Ministers.csv").expect("Failed to create file.");
    file.write_all("Name of Commissioner, Ministry, Geopolitical Zone".as_bytes()).expect("Failed to write header");
    for (i, name) in name.iter().enumerate() {
        let line = format!("\n{}, {}, {}", name, ministry[i], zone[i]);
        file.write_all(line.as_bytes()).expect("failed to write line.");
    }

    println!("Data has been merged into Convicted_Ministers.csv");
}
