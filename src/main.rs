// Used perplexity.ai to optimize it, I understand what is happening and what it added
use rand::Rng;

fn main() {
    // Used primarily for background characters
    let labels = [
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

    let mut rng = rand::rng();

    let sexuality_index = rng.random_range(0..labels.len());
    let romantic_index = rng.random_range(0..labels.len());

    let sexuality = labels[sexuality_index];
    let romantic = labels[romantic_index];

    // Handle if "demi" is rolled — assign an underlying orientation
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
    
    // Random Birthdates
    let month = rng.random_range(1..=12);
    let max_day = match month {
        // range based on the month that it lands on
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };
    let day = rng.random_range(1..=max_day); // the day chosen

    println!("random month: {}", month);
    println!("random day: {}", day);
    println!("visit [lgbtqia.wiki] for more information");
    println!("random sexuality: {}", resolved_sexuality);
    println!("random romantic: {}", resolved_romantic);
}


