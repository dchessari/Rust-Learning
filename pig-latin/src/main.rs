fn check_word (original: &str) -> String {
    // sort string into words that start with vowels and words that start with consts
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    // let cons = ['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l' 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'y', 'z'];

    for word in original.split_whitespace() {

    }
}

fn vowel_start() {
    // how to handle words that start with vowels

}

fn cons_start() {
    // how to handle words that start with consts

}

fn suffix() {
    // update original string to be in pig latin

}

fn main() {
    // define the string
    let mut original = String::from("I want to be six feet tall when I grow up");

    // print updated string
}





// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay is added,
// so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay).
// Keep in mind the details about UTF-8 encoding!