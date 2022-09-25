pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
    // 原地哈希
    let mut nums = nums;
    let n = nums.len();
    let mut res = vec![];
    for i in 0..n {
        let num = if nums[i] < 0 { - nums[i] } else { nums[i] };
        let index = num as usize - 1;
        if nums[index] < 0 {
            res.push(num);
        } else {
            nums[index] = - nums[index];
        }
    }
    res
}

#[test]
fn test() {
    assert_eq!(find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]), vec![2, 3]);
}

#[test]
fn test2() {
    assert_eq!(find_duplicates(vec![1, 2, 3, 4, 4]), vec![4]);
}

#[test]
fn test3() {
    assert_eq!(find_duplicates(vec![1]), vec![]);
}