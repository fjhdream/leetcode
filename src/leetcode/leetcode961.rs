use rand::prelude::*;
pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
    let mut rnd = thread_rng();
    let n = nums.len();
    let mut res = -1;
    loop {
        let index = rnd.gen_range(0..n);
        let other_index = rnd.gen_range(0..n);
        if index != other_index {
            if nums[index] == nums[other_index] {
                res = nums[index];
                break;
            }
        }
    }
    res
}

#[test]
fn test() {
    let ans = repeated_n_times(vec![5,1,5,2,5,3,5,4]);
    assert_eq!(ans, 5);
}


#[test]
fn test2() {
    let ans = repeated_n_times(vec![2,1,2,5,3,2]);
    assert_eq!(ans, 2);
}

#[test]
fn test3() {
    let ans = repeated_n_times(vec![8,3,2,3]);
    assert_eq!(ans, 3);
}