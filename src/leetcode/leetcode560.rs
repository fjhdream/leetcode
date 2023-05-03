use std::collections::HashMap;

pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut count = 0;
    let mut prefix = vec![0; nums.len() + 1];
    for i in 0..nums.len() {
        prefix[i + 1] = prefix[i] + nums[i];
    }
    let mut map = HashMap::new();
    map.insert(0, 1);
    for (i, v) in nums.iter().enumerate() {
        let key = prefix[i + 1] - k;
        if let Some(&val) = map.get(&key) {
            count += val;
        }
        *map.entry(prefix[i + 1]).or_insert(0) += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subarray_sum() {
        assert_eq!(subarray_sum(vec![1, 1, 1], 2), 2);
        assert_eq!(subarray_sum(vec![1, 2, 3], 3), 2);
    }

    #[test]
    fn test_subarray_sum_2() {
        assert_eq!(subarray_sum(vec![1, -1, 0], 0), 3);
    }
}
