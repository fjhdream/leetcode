pub fn find_the_distance_value(arr1: Vec<i32>, mut arr2: Vec<i32>, d: i32) -> i32 {
    arr2.sort();
    let mut res = 0;
    for num in arr1 {
        let idx = binary_search(&arr2, num);
        let mut ok = true;
        if idx == arr2.len() {
            ok &= num - arr2[idx - 1] > d;
        } else if idx == 0 {
            ok &= arr2[idx] - num > d;
        } else {
            ok &= num - arr2[idx - 1] > d;
            ok &= arr2[idx] - num > d;
        }
        if ok {
            res += 1;
        }
    }
    res
}

fn binary_search(arr: &Vec<i32>, target: i32) -> usize {
    let mut left = 0;
    let mut right = arr.len() - 1;
    while left <= right {
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            return mid;
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            if mid == 0 {
                return 0;
            }
            right = mid - 1;
        }
    }
    return left;
}

#[test]
fn test() {
    let ans = find_the_distance_value(vec![4, 5, 8], vec![10, 9, 1, 8], 2);
    assert_eq!(ans, 2);
}

#[test]
fn test2() {
    let ans = find_the_distance_value(vec![1, 4, 2, 3], vec![-4, -3, 6, 10, 20, 30], 3);
    assert_eq!(ans, 2);
}

#[test]
fn test3() {
    let ans = find_the_distance_value(vec![2, 1, 100, 3], vec![-5, -2, 10, -3, 7], 6);
    assert_eq!(ans, 1);
}

#[test]
fn test4() {
    let ans = find_the_distance_value(vec![-8, -7], vec![4, 10, -4, 5, 2], 55);
    assert_eq!(ans, 0);
}
