pub fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
    let max = a.max(b).max(c);
    let sum = a + b + c;
    if sum - max < max {
        return sum - max;
    } else {
        return sum / 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let ans = maximum_score(4, 4, 6);
        assert_eq!(ans, 7);
    }

    #[test]
    fn test2() {
        let ans = maximum_score(6, 2, 1);
        assert_eq!(ans, 3);
    }
}
