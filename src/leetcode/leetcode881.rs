pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
    people.sort_unstable();
    let mut left = 0usize;
    let mut right = people.len() - 1;
    let mut ans = 0;
    while left < right {
        if &people[left] + &people[right] <= limit {
            left += 1;
        } 
        right -= 1;
        ans += 1;
    }
    return if left == right { ans + 1 } else { ans }
}