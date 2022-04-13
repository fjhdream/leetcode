pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
    let (mut line, mut count) = (0, 0);
    for c in s.chars() {
        let width = widths[c as usize - 'a' as usize];
        if count + width > 100 {
            line += 1;
            count = width;
        } else {
            count += width;
        }
    }
    vec![line + 1, count]
}