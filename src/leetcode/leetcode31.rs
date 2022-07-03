pub fn next_permutation(nums: &mut Vec<i32>) {
    let n = nums.len();
    let mut i = n as i32 - 2;
    while i >= 0 && nums[i as usize] >= nums[i as usize + 1] {
        i -= 1;
    }

    if i >= 0 {
        let mut j = n as i32 - 1;
        while j >= i && nums[j as usize] <= nums[i as usize] {
            j -= 1;
        }
        nums.swap(i as usize, j as usize);
    }
    let mut start = (i + 1) as usize;
    let mut end = n - 1;
    while start < end {
        nums.swap(start, end);
        start += 1;
        end -= 1;
    }
}

#[test]
fn test() {
    let mut arr = vec![2,3,0,2,4,1];
    next_permutation(&mut arr);
    assert_eq!(arr, vec![2,3,0,4,1,2]);
}


#[test]
fn test2() {
    let mut arr = vec![3,2,1];
    next_permutation(&mut arr);
    assert_eq!(arr, vec![1,2,3]);
}

#[test]
fn test3() {
    let mut arr = vec![3];
    next_permutation(&mut arr);
    assert_eq!(arr, vec![3]);
}