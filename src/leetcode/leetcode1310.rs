pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = arr.len();
    let mut prefix = vec![0; n + 1];
    for i in 1..=n {
        prefix[i] = prefix[i-1] ^ arr[i-1];
    }
    let m = queries.len();
    let mut ans = vec![0; m];
    for i in 0..m {
        let left = queries[i][0] as usize;
        let right = queries[i][1] as usize;
        ans[i] = prefix[right + 1] ^ prefix[left];
    }
    ans
}

#[test]
pub fn test_example() {
    let arr = vec![16];
    let queries = vec![vec![0,0], vec![0,0], vec![0,0]];
    let ans = xor_queries(arr, queries);
    println!("{:?}", ans)
}