pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
    let m = rolls.len() as i32;
    let sum = mean * (n + m);
    let roll_sum = rolls.iter().sum::<i32>();
    let last_sum = sum - roll_sum;
    let round = last_sum / n;
    let last = last_sum % n;
    if last_sum > n * 6 || last_sum < n * 1 {
        return vec![];
    }
    let mut ans = vec![round; n as usize];
    for i in 0..last as usize {
        ans[i] += 1;
    }
    ans
}

#[test]
fn test() {
    println!("{:?}", missing_rolls(vec![3,2,4,3], 4, 2));
}

#[test]
fn test2() {
    println!("{:?}", missing_rolls(vec![1,5,6], 3, 4));
}

#[test]
fn test3() {
    println!("{:?}", missing_rolls(vec![1,2,3,4], 6, 4));
}

#[test]
fn test4() {
    println!("{:?}", missing_rolls(vec![1], 3, 1));
}