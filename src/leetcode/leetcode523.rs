use std::collections::HashMap;

#[allow(dead_code)]
pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
    let mut pre_sum = 0;
    let mut map = HashMap::new();
    map.insert(0, -1);
    for (i, &v) in nums.iter().enumerate() {
        pre_sum = (pre_sum + v % k) % k;
        if map.contains_key(&pre_sum) {
            if i as i32 - map.get(&pre_sum).unwrap() >= 2 {
                return true;
            }
        } else {
            map.insert(pre_sum, i as i32);
        }
    }
    false
}

#[test]
pub fn test_example() {
    let numbs = vec![23,2,6,4,7];
    let k = 13;
    let ans = check_subarray_sum(numbs, k);
    assert_eq!(ans, false);
}