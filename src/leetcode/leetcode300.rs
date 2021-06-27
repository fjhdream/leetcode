// 二分搜索算法， 耐心排序 patience sorting
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut top = vec![0; nums.len()];
    let mut piles = 0;
    for &num in nums.iter() {
        let mut left = 0;
        let mut right = piles;
        while left < right {
            let mid = (left + right) / 2;
            if top[mid] > num {
                right = mid;
            } else if top[mid] < num {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        if left == piles {
            piles += 1;
        }

        top[left] = num;
    }

    piles as i32
}

#[test]
fn test_emp() {
    let nums = vec![10,9,2,5,3,7,101,18];
    let ans = length_of_lis(nums);
    assert_eq!(ans, 4);
}