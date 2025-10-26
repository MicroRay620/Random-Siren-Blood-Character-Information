// Used perplexity.ai to optimize it, I understand what is happening and what it added
use rand::Rng;
fn main() {
    // Used primarily for background characters
    let labels = [ // Random Labels, may add more
        "straight", // attraction to the opposite gender
        "gay", // attraction to the same gender (mlm exclusively)
        "lesbian", // attraction to the same gender (wlw exclusively)
        "bi", "pan", "omni", // attraction to one or more genders
        "demi", // attraction based on how long a person knew the other
        "cetero", // attraction to non-binary or genderqueer people
        "fin", // attraction to femininity
        "abro", // attraction to masculinity
        "ace", // little or no attraction
    ];
    let gender_labels = [
        "cis man",
        "cis woman",
        "trans man",
        "trans woman",
        "genderfluid",
        "demigirl",
        "demiboy",
        "non-binary",
        "bigender",
        "pangender"
    ];
    let mut rng = rand::rng();
    // Random Label Ranges
    let sexuality_index = rng.random_range(0..labels.len());
    let romantic_index = rng.random_range(0..labels.len());
    // The random determiners
    let sexuality = labels[sexuality_index];
    let romantic = labels[romantic_index];
    // Handle if "demi" is rolled // assign an underlying orientation
    let resolved_sexuality = if sexuality == "demi" {
        let demi_target = labels[rng.random_range(0..labels.len())];
        format!("demi ({})", demi_target)
    } else {
        sexuality.to_string()
    };
    let resolved_romantic = if romantic == "demi" {
        let demi_target = labels[rng.random_range(0..labels.len())];
        format!("demi ({})", demi_target)
    } else {
        romantic.to_string()
    };
    let gender_index = rng.random_range(0..gender_labels.len());
    let gender = gender_labels[gender_index];
    // Random Birthdates
    let month = rng.random_range(1..12);
    let max_day = match month {
        // range based on the month that it lands on
        2 => 28, // Add a chance for it to be 29 // Months with < 30 days
        4 | 6 | 9 | 11 => 30, // Months with 30 days
        _ => 31, // Every other month
    };
    let age = rng.random_range(20..30);
    let day = rng.random_range(1..max_day); // the day chosen
    println!("random month: {}", month);
    println!("random day: {}", day);
    println!("random age: {}", age);
    println!("visit [lgbtqia.wiki] for more information");
    println!("random sexuality: {}", resolved_sexuality);
    println!("random romantic: {}", resolved_romantic);
    println!("random gender: {}", gender);
}
