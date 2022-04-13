use std::collections::HashMap;

pub fn unique_morse_representations(words: Vec<String>) -> i32 {
    let morse:[&str;26] = [
        ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
        "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--..",
    ];
    let mut m = HashMap::new();
    for word in words {
        let mut s = String::new();
        for c in word.chars() {
            s.push_str(&morse[c as usize - 'a' as usize]);
        }
        m.insert(s, 1);
    }
    m.len() as i32
}