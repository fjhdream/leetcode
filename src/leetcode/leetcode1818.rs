pub fn min_absolute_sum_diff(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mod_num = 1000000007;
    let mut num = 0;
    let mut maxn = 0;
    let mut res = nums1.clone();
    res.sort();
    let n = nums1.len();
    for i in 0..n {
        let diff = (nums2[i] - nums1[i]).abs();
        num = (diff + num) % mod_num;
        let j = binary_search(&res, nums2[i]);
        if j < n {
            maxn = maxn.max(diff - (res[j] - nums2[i]));
        }
        if j > 0 {
            maxn = maxn.max(diff - (nums2[i] - res[j-1]));
        }
    }
    
    (num - maxn + mod_num) % mod_num
}

fn binary_search(nums : &Vec<i32>, target: i32) -> usize {
    let mut left = 0usize;
    let mut right = nums.len();
    if nums[right-1] < target {
        return right;
    }
    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] >= target {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

#[test]
fn test_binary_search() {
    let mut nums1 = vec![1,10,4,4,2,7];
    let nums2 = vec![2, 3, 5];
    // let ans = min_absolute_sum_diff(nums1, nums2);
    nums1.sort();
    let ans = binary_search(&nums1, 6);
    assert_eq!(ans, 1)
}