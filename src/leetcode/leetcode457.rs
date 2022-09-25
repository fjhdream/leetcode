pub fn circular_array_loop(mut nums: Vec<i32>) -> bool {
    let n = nums.len();
    for i in 0.. n {
        if nums[i] == 0 {
            continue;
        }
        let mut slow = i;
        let mut fast = next(&nums, next(&nums, i));
        // 快慢指针非0且同向
        while is_same_direction(&nums, slow, fast) 
            && is_same_direction(&nums, slow, next(&nums, fast)) {
            if slow == fast {
                // 判断环长度是否为1
                if slow != next(&nums, slow) {
                    return true;
                } else { 
                    break;
                }
            }
                
            slow = next(&nums, slow);
            fast = next(&nums, next(&nums, fast));
        }

        // 如果已经便利过的话, 将nusm[i] 置为0 标记已经遍历过
        let mut add = i;
        while nums[add] * nums[next(&nums, add)] > 0 {
            let tmp = add;
            add = next(&nums, add);
            nums[tmp] = 0;
        }
    }

    false
}

fn next(nums: &Vec<i32>, cur: usize) -> usize {
    let n = nums.len() as i32;
    return ((cur as i32  + nums[cur] % n  + n) % n)  as usize; 
}

fn is_same_direction(nums: &Vec<i32>, slow: usize, fast: usize) -> bool {
    return nums[slow] * nums[fast] > 0
}

#[test]
fn test_example() {
    let nums = vec![2, -1, 1, 2, 2];
    let ans = circular_array_loop(nums);
    assert_eq!(ans, true);
}