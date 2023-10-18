use std::collections::BinaryHeap;

pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
    if nums.is_empty() {
        return 0;
    }
    let mut heap = BinaryHeap::new();
    for num in nums {
        heap.push(num);
    }
    let mut count = 0;
    let mut ans = 0_i64;
    while count < k {
        let num = heap.pop().unwrap();
        ans += num as i64;
        heap.push((num as f64 / 3.0).ceil() as i32);
        count += 1;
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![10, 10, 10, 10, 10];
        let k = 5;
        let res = max_kelements(nums, k);
        assert_eq!(res, 50);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 10, 3, 3, 3];
        let k = 3;
        let res = max_kelements(nums, k);
        assert_eq!(res, 17);
    }
}
