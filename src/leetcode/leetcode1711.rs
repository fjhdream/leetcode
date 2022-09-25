use std::collections::HashMap;

pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
    let mod_num = 1000000007;
    let mut map = HashMap::new();
    let max_num = deliciousness.iter().max().unwrap().to_owned() * 2;
    let mut ans = 0;
    for deliciousnes in deliciousness.iter() {
        for big_meal in (2..=max_num).take_while(|item| (*item).count_ones() == 1u32) {
            let another = big_meal - deliciousnes;
            if map.contains_key(&another) {
                ans += map.get(&another).unwrap();
                ans %= mod_num;
            }
        }
        *map.entry(deliciousnes).or_insert(0) += 1;
    }
    ans
}

#[test]
fn test_example() {
    let deliciousness = vec![1,1,1,3,3,3,7];
    let ans = count_pairs(deliciousness);
    assert_eq!(ans, 15);
}