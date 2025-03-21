fn squeel_piggy (original: &str) -> String {
    // sort string into words that start with vowels and words that start with consts
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let Some(first_char) = word.chars().next() {
        if first_char.contains(&vowels[..]) {
            format!("{}-hay", original)
            else { format!("{}-{}ay", original, first_char)
        }
    }
}

fn main() {
    // define the string
    let mut original = String::from("I want to be six feet tall when I grow up");

    // print updated string
}





// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay is added,
// so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay).
// Keep in mind the details about UTF-8 encoding!