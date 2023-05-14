use std::collections::HashSet;

pub fn find_max_k(nums: Vec<i32>) -> i32 {
    let mut max_k = -1;
    let mut set = HashSet::new();
    for &num in nums.iter() {
        if num < 0 {
            set.insert(num);
        }
    }
    for num in nums {
        if num > 0 && num > max_k && set.contains(&-num) {
            max_k = num;
        }
    }
    max_k
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_k() {
        let nums = vec![3, 2, -2, 5, -3];
        assert_eq!(3, find_max_k(nums));
        let nums = vec![-1, 10, 6, 7, -7, 1];
        assert_eq!(7, find_max_k(nums));
        let nums = vec![-10, 8, 6, 7, -2, -3];
        assert_eq!(-1, find_max_k(nums));
    }
}
