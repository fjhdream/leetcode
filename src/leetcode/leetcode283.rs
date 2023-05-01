pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut left = 0;
    let mut right = 0;
    while right < nums.len() {
        if nums[right] != 0 {
            nums.swap(left, right);
            left += 1;
        }
        right += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut nums = vec![1, 0, 0, 3, 12];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn test2() {
        let mut nums = vec![0];
        move_zeroes(&mut nums);
        assert_eq!(vec![0], nums);
    }

    #[test]
    fn test3() {
        let mut nums = vec![0, 1];
        move_zeroes(&mut nums);
        assert_eq!(vec![1, 0], nums);
    }
}
