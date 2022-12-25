pub fn minimum_boxes(n: i32) -> i32 {
    let mut ans = 0;
    let mut max_n = 0;
    let mut i = 1;
    while max_n + i + ans <= n {
        ans += i;
        max_n += ans;
        i += 1;
    }
    let mut j = 1;
    while max_n < n {
        ans += 1;
        max_n += j;
        j += 1;
    }
    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let ans = minimum_boxes(3);
        assert_eq!(ans, 3);
    }

    #[test]
    fn test2() {
        let ans = minimum_boxes(10);
        assert_eq!(ans, 6);
    }
}
