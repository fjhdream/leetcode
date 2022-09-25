pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mod_num = 1000000007;

    let mut map = HashMap::new();
    let powers: Vec<i32> = (0..=21).map(|i| 1 << i).collect();
    let mut ans = 0;
    for delicious in deliciousness.iter() {
        for big_meal in powers.iter()  {
            let another = big_meal - delicious;
            if map.contains_key(&another) {
                ans += map.get(&another).unwrap();
                ans %= mod_num;
            }
        }
        *map.entry(delicious).or_insert(0) += 1;
    }
    ans
}

#[test]
fn test_example() {
    let deliciousness = vec![149,107,1,63,0,1,6867,1325,5611,2581,39,89,46,18,12,20,22,234];
    let ans = count_pairs(deliciousness);
    assert_eq!(ans, 12);
}