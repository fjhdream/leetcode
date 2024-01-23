pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
    let mut res = -1;
    let n = nums.len();
    for i in 0..n {
        for j in i + 1..n {
            let length = j - i + 1;
            if nums[j] - nums[i] == (length - 1) as i32 % 2 {
                res = res.max(length as i32);
            } else {
                break;
            }
        }
    }
    res
}

pub fn alternating_subarray2(nums: Vec<i32>) -> i32 {
    let mut res = -1;
    let mut frist_index = 0;
    let n = nums.len();
    for i in 1..n {
        let length = i - frist_index + 1;
        if nums[i] - nums[frist_index] == (length - 1) as i32 % 2 {
            res = res.max(length as i32);
        } else {
            if nums[i] - nums[i - 1] == 1 {
                frist_index = i - 1;
                res = res.max(2);
            } else {
                frist_index = i;
            }
        }
    }
    res
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(alternating_subarray(vec![2, 3, 4, 3, 4]), 4);
    }
}
