use std::collections::HashMap;

pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, (usize, usize, usize)> = HashMap::new();
    for i in 0..nums.len() {
        let key = nums[i];
        if map.contains_key(&key) {
            map.get_mut(&key).unwrap().0 += 1;
            map.get_mut(&key).unwrap().2 = i;
        } else {
            map.insert(key, (1, i, i));
        }
    }

    let mut max_num = 0;
    let mut ans = 0;
    for (_, val) in map.iter() {
        if val.0 > max_num {
            max_num = val.0;
            ans = val.2 - val.1 + 1
        } else if val.0 == max_num {
            ans = ans.min(val.2 - val.1 + 1)
        }
    }

    ans as i32
}

#[test]
pub fn test_example() {
    let nums = vec![1, 2, 2, 3, 1];
    let ans = find_shortest_sub_array(nums);
    assert_eq!(ans, 2);
}