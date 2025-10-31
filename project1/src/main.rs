use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    // UEFA countries data

    let mut countries = vec!["Albania", "Andorra", "Armenia", "Austria", "Azerbaijan", "Belarus", "Belgium", "Bosnia", "Bulgaria", "Croatia", "Cyprus", "Czechia", "Denmark", "England", "Estonia", "Faroe Islands", "Finland", "France", "Georgia", "Germany", "Gibraltar", "Greece", "Hungary", "Iceland", "Israel", "Italy", "Kazakhstan", "Kosovo", "Latvia", "Liechtenstein", "Lithuania", "Luxembourg", "Malta", "Moldova", "Montenegro", "Netherlands", "North Macedonia", "Northern Ireland", "Norway", "Poland", "Portugal", "Republic of Ireland", "Romania", "Russia", "San Marino", "Scotland", "Serbia", "Slovakia", "Slovenia", "Spain", "Sweden", "Switzerland", "Türkiye", "Ukraine", "Wales"];
    
    // Phase 1 Draw 11 groups of 5

    let mut rng = thread_rng();
    countries.shuffle(&mut rng);

    let mut groups: Vec<Vec<&str>> = countries
        .chunks(5)
        .map(|chunk| chunk.to_vec())
        .collect();

    for (i, group) in groups.iter().enumerate() {
        println!("Group {}:", i + 1);
        for team in group {
            println!(" - {}", team);
        }
        println!();
    }

    // Phase 1 - Final Standings

    for group in groups.iter_mut() {
        group.shuffle(&mut rng);
    }

    for (i, group) in groups.iter().enumerate() {
        println!("🏆 Group {} Final Standings:", i + 1);
        for (position, team) in group.iter().enumerate() {
            println!("  {}. {}", position + 1, team);
        }
        println!(); // blank line between groups
    }

    // Phase 3 - Qualifiers (Top 2)

    let mut qualified: Vec<&str> = Vec::new();

    for group in &groups {
        qualified.push(group[0]); // 1st place
        qualified.push(group[1]); // 2nd place
}


}