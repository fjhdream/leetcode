use std::collections::HashSet;
pub fn fair_candy_swap(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let sum_a: i32 = a.iter().sum();
    let sum_b: i32 = b.iter().sum();
    let diff = (sum_a - sum_b) / 2;
    let mut set = HashSet::new();
    for num_a in a.iter() {
        set.insert(num_a);
    }
    let mut ans = vec![0;2];
    for num_b in b.iter() {
        let a = num_b + diff;
        if set.contains(&a) {
            ans[0] = a;
            ans[1] = *num_b;
            break;
        }
    }
    ans
}

#[test]
fn test_example() {
    let a = vec![1,1];
    let b = vec![2,2];
    let ans = fair_candy_swap(a, b);
    println!("{:?}", ans);
}