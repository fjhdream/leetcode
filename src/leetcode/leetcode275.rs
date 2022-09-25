#[allow(dead_code)]
pub fn h_index(citations: Vec<i32>) -> i32 {
    let n = citations.len();
    let (mut left, mut right) = (0, n);
    while left < right {
        let mid = left + (right - left) / 2;
        if citations[mid] as usize >= n - mid {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    (n - left) as i32
}

#[test]
fn test() {
    let nums = vec![100];
    let ans = h_index(nums);
    assert_eq!(ans, 1);
}