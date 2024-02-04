use std::{fs::File, collections::HashMap};
fn fre_count() {
    let test_str: String =
        String::from("The quick brown fox jumps over the lazy dog. The dog barks, and the fox runs away.");

    let mut my_dict: HashMap<String, i32> = HashMap::new();

    let mut x: String = String::new();
    for i in test_str.chars() {
        if i.is_whitespace() {
            // Check if the word is already in the HashMap
            let entry = my_dict.entry(x.clone()).or_insert(0);
            *entry += 1;

            x.clear();
        } else {
            x.push(i);
        }
    }
    println!("{:?}",my_dict);
    if !x.is_empty() {
        let entry = my_dict.entry(x.clone()).or_insert(0);
        *entry += 1;
    }

    for (word, frequency) in &my_dict {
        println!("Word: {}, Frequency: {}", word, frequency);
    }
}
fn main() {
fre_count();
}
