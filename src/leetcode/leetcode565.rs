pub fn array_nesting(nums: Vec<i32>) -> i32 {
    let (mut ans, n) = (0, nums.len());
    let mut visited = vec![false; n];
    let mut i = 0usize;
    while i < n {
        let mut count = 0;
        while !visited[i] {
            visited[i] = true;
            i = nums[i] as usize;
            count += 1;
        }
        ans = ans.max(count);
        i += 1;
    }
    ans
}

#[test]
fn test() {
    let ans = array_nesting(vec![5,4,0,3,1,6,2]);
    assert_eq!(ans, 4);
}