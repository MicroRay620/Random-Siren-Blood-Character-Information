use rand::Rng;
fn main() {
    // Primarily use this for background characters
    // Major characters don't use this as much.
    let label_list = ["straight", "gay", "lesbian", "bi", "pan", "omni", "demi", "cetero" /* attraction to non-binary and gender queer people */, "fin" /* attraction to femininity */, "abro" /* attraction to masculinity */ , "ace"];
    let mut rng = rand::rng();
    let random_sexuality: usize = rng.random_range(0..label_list.len());
    let random_romantic: usize = rng.random_range(0..label_list.len());
    let random_month: i32 = rng.random_range(1..12);
    let random_day: i32 = rng.random_range(0..30);
    println!("random month: {}", random_month);
    if random_day == 0 {
        println!("random day: 31 [Only count if month has 31 days]");
    } else {
        println!("random day: {}", random_day);
    }
    println!("visit [lgbtqia.wiki] for more information"); 
    println!("random sexuality: {}", label_list[random_sexuality]);
    println!("random romantic: {}", label_list[random_romantic]);
}
