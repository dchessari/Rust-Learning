fn squeel_piggy (original: &str) -> String {

    // sort string into words that start with vowels and words that start with consts
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    // let cons = ['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'y', 'z'];

    if let Some(first_char) = original.chars().next() {
        if vowels.contains(&first_char.to_ascii_lowercase()) {
            format!("{}-hay", original)
        } else {
            let remaining: String = original.chars().skip(1).collect();
            format!("{}-{}ay", remaining, first_char)
        }
    } else {
        original.to_string()
    }
}

fn main() {
    // define the string
    let mut original = String::from("I want to be six feet tall when I grow up");

    // re-assemble words post-processing
    let piggy: String = original
        .split_whitespace()
        .map(squeel_piggy)
        .collect::<Vec<String>>()
        .join(" ");

    // print updated string
    println!("Original: {}", original);
    println!("Like a pig: {}", piggy);
}





// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay is added,
// so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay).
// Keep in mind the details about UTF-8 encoding!