pub fn reverse_vowels(s: String) -> String {
    let mut bytes = s.into_bytes();
    let n = bytes.len();
    let mut l = 0;
    let mut r = n - 1;
    while l < r {
        while l < n && !is_voewl(bytes[l]) {
            l += 1;
        }
        while r > 0 && !is_voewl(bytes[r]) {
            r -= 1;
        }
        if l < r {
            bytes.swap(l, r);
            l += 1;
            r -= 1;
        }
    }
    String::from_utf8(bytes).unwrap()
}

fn is_voewl(letter: u8) -> bool {
    let letter = letter as char;
    return letter == 'a' || letter == 'e' || letter == 'i' || letter == 'o' || letter == 'u'
        || letter == 'A' || letter == 'E' || letter == 'I' || letter == 'O' || letter == 'U';    
}

#[test]
fn test_example() {
    let s = String::from("leetcode");
    let ans = reverse_vowels(s);
    assert_eq!(ans, "leotcede");
}