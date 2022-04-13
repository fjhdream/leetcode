pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    for &letter in letters.iter() {
        if letter > target {
            return letter;
        }
    }
    return letters[0];
}