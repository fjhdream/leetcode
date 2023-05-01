pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();
    let mut ans = vec![];
    let n = nums.len();
    for first in 0..n {
        if first > 0 && nums[first] == nums[first - 1] {
            continue;
        }
        let mut third = n - 1;
        let target = -nums[first];
        for second in first+1..n {
            if second > first + 1 && nums[second] == nums[second - 1] {
                continue;
            }
            while second < third && nums[second] + nums[third] > target {
                third -= 1;
            }
            if second == third {
                break;
            }
            if nums[second] + nums[third] == target {
                ans.push(vec![nums[first], nums[second], nums[third]]);
            }  
        }
        
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let ans = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(three_sum(nums), ans);
    }
}