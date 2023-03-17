pub fn answer_queries(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
    let mut sums = vec![0; nums.len() + 1];
    nums.sort();
    for i in 0..nums.len() {
        sums[i+1] = sums[i] + nums[i]; 
    }
    let mut ans = vec![0; queries.len()];
    for (idx, query) in queries.iter().enumerate() {
        let find = sums.binary_search(query);
        match find {
            Ok(l) => {ans[idx] = l as i32},
            Err(l) => {ans[idx] = l as i32 - 1},
        }
    }
    return ans;
}

#[test]
fn test() {
    let ans = answer_queries(vec![4,5,2,1],vec![3,10,21]);
    assert_eq!(ans, vec![2,3,4]);
}