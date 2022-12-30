fn valid_isbn10(isbn: &str) -> bool {
    let chs = isbn.chars().collect::<Vec<_>>();
    if chs.len() != 10 {
        return false;
    }
    if chs.iter().enumerate().any(|(i, &c)| {
        if i == 9 {
            c != 'X' && !c.is_digit(10)
        } else {
            !c.is_digit(10)
        }
    }) {
        return false;
    }

    let sum = chs.iter().enumerate().fold(0i32, |acc, (idx, &ch)| {
        if idx == 9 {
            if ch == 'X' {
                acc + 10 * (idx as i32 + 1)
            } else {
                acc + ch.to_digit(10).unwrap() as i32 * (idx as i32 + 1)
            }
        } else {
            acc + ch.to_digit(10).unwrap() as i32 * (idx as i32 + 1)
        }
    });

    sum % 11 == 0
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::valid_isbn10;

    fn dotest(isbn: &str, expected: bool) {
        let actual = valid_isbn10(isbn);
        assert!(
            actual == expected,
            "Test failed with isbn = {isbn}\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn sample_tests() {
        dotest("1112223339", true);
        dotest("048665088X", true);
        dotest("1293000000", true);
        dotest("1234554321", true);
        dotest("1234512345", false);
        dotest("1293", false);
        dotest("X123456788", false);
        dotest("ABCDEFGHIJ", false);
        dotest("XXXXXXXXXX", false);
        dotest("123456789T", false);
    }
}
