#[allow(dead_code)]
pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    let sum: i32 = nums.iter().sum();
    let neg = sum - target;
    if neg < 0 || (neg & 1) !=0 {
        return 0;
    }

    let n = nums.len();
    let neg_num = neg as usize / 2;
    let mut dp = vec![vec![0; neg_num + 1]; n + 1];
    dp[0][0] = 1;
    for i in 1..=n {
        let num = nums[i-1];
        for j in 0..=neg_num {
            dp[i][j] = dp[i-1][j];
            if j as i32 >= num {
                dp[i][j] += dp[i-1][j-num as usize];
            }
        }
    }
    dp[n][neg_num]
}

static mut COUNT: i32 = 0;

#[allow(dead_code)]
pub fn find_target_sum_ways2(nums: &Vec<i32>, target: i32) -> i32 {
    dfs(&nums, target, 0, 0);
    unsafe {
        return COUNT;
    }
}

fn dfs(nums: &Vec<i32>, target:i32, index:usize, sum: i32) {
    if index == nums.len() { 
        if target == sum {
            unsafe {
                COUNT += 1;
            }
        } 
    } else {
        dfs(nums, target, index + 1, sum - nums[index]);
        dfs(nums, target, index + 1, sum + nums[index]);
    }

}

#[test]
fn test_example() {
    let nums = vec![1,1,1,1,1];
    let target = 3;
    let ans2 = find_target_sum_ways2(&nums, target);
    let ans = find_target_sum_ways(nums, target);
    assert_eq!(ans, 5);
    assert_eq!(ans2, ans);
}