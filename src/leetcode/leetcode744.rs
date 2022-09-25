pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    for &letter in letters.iter() {
        if letter > target {
            return letter;
        }
    }
    return letters[0];
}

pub fn next_greatest_letter2(letters: Vec<char>, target: char) -> char {
    let (mut left, mut right) = (0, letters.len() - 1);
    while left <= right {
        let mid = left + (right - left) / 2;
        if letters[mid] == target {
            left = mid + 1;
        } else if letters[mid] < target {
            left = mid + 1;
        } else {
            if mid == 0 {
                return letters[0];
            }
            right = mid - 1;
        }
    }
    return letters[left % letters.len()];
}

#[test]
fn test() {
    let ans = next_greatest_letter2(vec!['c', 'f', 'j'], 'a');
    assert_eq!(ans, 'c');
}

#[test]
fn test2() {
    let ans = next_greatest_letter2(vec!['c', 'f', 'j'], 'c');
    assert_eq!(ans, 'f');
}

#[test]
fn test3() {
    let ans = next_greatest_letter2(vec!['c', 'f', 'j'], 'f');
    assert_eq!(ans, 'j');
}

#[test]
fn test4() {
    let ans = next_greatest_letter2(vec!['c', 'f', 'j'], 'g');
    assert_eq!(ans, 'j');
}

#[test]
fn test5() {
    let ans = next_greatest_letter2(vec!['c', 'f', 'j'], 'j');
    assert_eq!(ans, 'c');
}
