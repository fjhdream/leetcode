pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
    let (mut left, mut right) = (0, arr.len() - 1);
    while left <= right {
        let mid = left + (right - left) / 2;
        if arr[mid] > arr[mid - 1] && arr[mid] > arr[mid + 1] {
            return mid as i32;
        } else if arr[mid] < arr[mid + 1] {
            left = mid;
        } else {
            right = mid;
        }
    }
    return -1;
}

#[test]
fn test() {
    let ans = peak_index_in_mountain_array(vec![0, 1, 0]);
    assert_eq!(ans, 1);
}

#[test]
fn test2() {
    let ans = peak_index_in_mountain_array(vec![0,10,5,2]);
    assert_eq!(ans, 1);
}

#[test]
fn test3() {
    let ans = peak_index_in_mountain_array(vec![24,69,100,99,79,78,67,36,26,19]);
    assert_eq!(ans, 2);
}

#[test]
fn test4() {
    let ans = peak_index_in_mountain_array(vec![3,5,3,2,0]);
    assert_eq!(ans, 1);
}

#[test]
fn test5() {
    let ans = peak_index_in_mountain_array(vec![24,100,99,79,78,67,36,26,19,18]);
    assert_eq!(ans, 1);
}