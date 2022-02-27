pub fn optimal_division(nums: Vec<i32>) -> String {
    let n = nums.len();
    if n == 1 {
        return nums[0].to_string();
    } else if n == 2 {
        return nums[0].to_string() + "/" + &nums[1].to_string();
    } else {
        return nums[0].to_string() + "/" + "(" + &nums[1..n].iter().map(|num| (*num).to_string()).collect::<Vec<_>>().join("/") + ")";
    }
}

#[test]
fn test() {
    let ans = optimal_division(vec![1000, 100, 10, 2]);
    assert_eq!(ans, "1000/(100/10/2)");
}