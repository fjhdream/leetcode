use std::collections::HashMap;

pub fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
    let mut freq = HashMap::new();
    for num in &nums {
        *freq.entry(*num).or_insert(0) += 1;
    }
    let mut ans = nums.iter().sum();
    let mut k = k as i32;
    for i in -100..0 {
        if freq.contains_key(&i) {
            let ops = k.min(freq.get(&i).unwrap().to_owned());
            ans += (-i) * ops * 2;
            freq.insert(i, freq.get(&i).unwrap().to_owned() - ops);
            *freq.entry(-i).or_default() += ops;
            k -= ops;
            if k == 0 {
                break;
            }
        }
    }
    if k > 0 && k % 2 == 1 && !freq.contains_key(&0){
        for i in 1..=100 {
            if freq.contains_key(&i) {
                ans -= i * 2;
                break;
            }
        }
    }
    ans
}

#[test]
fn test_example() {
    let numbs = vec![3,-1,0,2];
    let k = 3;
    let ans = largest_sum_after_k_negations(numbs, k);
    assert_eq!(ans, 6);
}

#[test]
fn test_example2() {
    let numbs = vec![-8,3,-5,-3,-5,-2];
    let k = 6;
    let ans = largest_sum_after_k_negations(numbs, k);
    assert_eq!(ans, 22);
}
