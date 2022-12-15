pub fn min_operations(s: String) -> i32 {
    let chs = s
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as i32)
        .collect::<Vec<_>>();
    let mut num10 = 0;
    let mut num01 = 0;
    let mut start = 0;
    for i in chs {
        if i != start {
            num01 += 1;
        }
        if i != 1 - start {
            num10 += 1;
        }
        start = 1 - start;
    }
    num01.min(num10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let ans = min_operations(String::from("0100"));
        assert_eq!(ans, 1);
    }

    #[test]
    fn test2() {
        let ans = min_operations(String::from("1111"));
        assert_eq!(ans, 2);
    }

    #[test]
    fn test3() {
        let ans = min_operations(String::from("10"));
        assert_eq!(ans, 0);
    }
}
