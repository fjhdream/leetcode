pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let set = nums
        .iter()
        .cloned()
        .collect::<std::collections::HashSet<i32>>();
    let mut max = 0;
    for &value in set.iter() {
        if set.contains(&(value - 1)) {
            continue;
        }
        let mut cur = 1;
        let mut value = value;
        while set.contains(&(value + 1)) {
            cur += 1;
            value = value + 1;
        }
        max = max.max(cur);
    }
    max
}
