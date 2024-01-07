fn get_longest_word(sentence: String) -> String {
    let mut longest_word = String::new();
    for word in sentence.split_whitespace() {
        if word.len() > longest_word.len() {
            longest_word = word.to_string();
        }
    }
    longest_word
}

fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();
    // Use slicing to get the first three characters of the sentence
    println!("{}", &sentence[0..=4]);

    // concatenate using format!
    let description = format!("Title: Quick story\n{}", sentence);
    println!("{}", description);

    // iterate over the characters in the sentence
    // for c in sentence.chars() {
    //     match c {
    //         'a' | 'e' | 'i' | 'o' | 'u' => println!("Got a vowel! {}", c),
    //         _ => println!("{}", c), //continue,
    //     }
    // }

    // iterate over the characters in the sentence
    let mut num_vowels = 0;
    for c in sentence.chars() {
        match c {
            // 'a' | 'e' | 'i' | 'o' | 'u' => println!("Got a vowel!"),
            'a' | 'e' | 'i' | 'o' | 'u' => num_vowels += 1,
            _ => continue,
        }
    }
    println!("The number of vowelse: {}", num_vowels);

    // Split and collect into a vector
    // let words: Vec<&str> = sentence.split_whitespace().collect();
    let words = sentence.split(' ').collect::<Vec<&str>>();
    println!("{:?}", words);

    let reversed = sentence.chars().rev().collect::<String>();
    println!("{}", reversed);

    let longest_word = get_longest_word(sentence);
    println!("{}", longest_word);
}
