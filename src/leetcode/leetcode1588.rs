pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut prefix_sum = vec![0; n + 1];
    for i in 0..n {
        prefix_sum[i + 1] = prefix_sum[i] + arr[i];
    }
    let mut ans = 0;
    for start in 0..n {
        for len in (1..=n).step_by(2) {
            if start + len > n {
                break;
            }
            let end = start + len - 1;
            ans += prefix_sum[end + 1] - prefix_sum[start];
        }
    }
    ans
}

#[test]
fn test_example() {
    let arr = vec![1,4,2,5,3];
    let ans = sum_odd_length_subarrays(arr);
    assert_eq!(ans, 58);
}