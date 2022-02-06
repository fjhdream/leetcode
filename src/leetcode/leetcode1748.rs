use std::collections::HashMap;

pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, usize> = HashMap::new();
    for num in nums {
        *map.entry(num).or_default() += 1;
    }
    let mut ans = 0;
    for (key, value) in map {
        if value == 1 {
            ans += key;
        }
    }
    ans
}

#[test]
fn test() {
    let ans = sum_of_unique(vec![1,2,3,2]);
    assert_eq!(ans, 4);
}