pub fn min_elements(nums: Vec<i32>, limit: i32, goal: i32) -> i32 {
    let tmp_sum: i64 = nums.iter().map(|val| *val as i64).sum();
    let diff = (goal as i64 - tmp_sum).abs();
    if diff == 0 {
        return 0;
    } else {
        return (diff / limit as i64 + if diff % limit as i64 == 0 { 0 } else { 1 }) as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let ans = min_elements(vec![1, -1, 1], 3, -4);
        assert_eq!(ans, 2);
    }

    #[test]
    fn test2() {
        let ans = min_elements(vec![1, -10, 9, 1], 100, 0);
        assert_eq!(ans, 1);
    }

    fn test3() {
        let ans = min_elements(vec![1, -1, 1], 3, 1);
        assert_eq!(ans, 0);
    }
}
