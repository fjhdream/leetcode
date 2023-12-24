pub fn minimum_perimeter(needed_apples: i64) -> i64 {
    let (mut left, mut right, mut ans) = (1_i64, 100000_i64, 0_i64);
    while left <= right {
        let mid = (left + right) / 2;
        let apples = 2 * mid * (mid + 1) * (mid * 2 + 1);
        if apples >= needed_apples {
            ans = mid;
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    8 * ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1954() {
        assert_eq!(minimum_perimeter(1), 8);
    }

    #[test]
    fn test1() {
        assert_eq!(minimum_perimeter(1000000000), 5040);
    }
}
