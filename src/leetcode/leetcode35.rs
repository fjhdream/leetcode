pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0, nums.len() - 1);
    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] > target {
            if mid == 0{
                return 0;
            }
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    return left as i32;
}

#[test]
fn test() {
    let ans = search_insert(vec![1, 3, 5, 6], 5);
    assert_eq!(ans, 2);
}

#[test]
fn test2() {
    let ans = search_insert(vec![1, 3, 5, 6], 7);
    assert_eq!(ans, 4);
}

#[test]
fn test3() {
    let ans = search_insert(vec![1, 3, 5, 6], 0);
    assert_eq!(ans, 0);
}

#[test]
fn test4() {
    let ans = search_insert(vec![1, 3, 5, 7], 6);
    assert_eq!(ans, 3);
}

#[test]
fn test5() {
    let ans = search_insert(vec![1, 3, 5, 7], 2);
    assert_eq!(ans, 1);
}

#[test]
fn test6() {
    let ans = search_insert(vec![1, 3], 0);
    assert_eq!(ans, 0);
}