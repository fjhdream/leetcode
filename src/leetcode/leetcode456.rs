pub fn find132pattern(nums: Vec<i32>) -> bool {
    let mut stack = Vec::new();
    let n = nums.len();
    stack.push(nums[n-1]);
    let mut max_k = i32::MIN;
    for i in (0..n-1).rev() {
        let num = nums[i];
        if num < max_k {
            return true;
        }

        while !stack.is_empty() && stack.last().unwrap() < &num {
            if let Some(pop_num) = stack.pop(){
                max_k = max_k.max(pop_num);
            }
        }

        if num >= max_k {
            stack.push(num);
        }
    }
    false
}

#[test]
pub fn test_example() {
    let nums = vec![3,1,4,2];
    let ans = find132pattern(nums);
    println!("the ans is {}", ans);
    assert_eq!(ans, true);
}