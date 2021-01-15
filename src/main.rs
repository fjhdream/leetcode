use leetcode::leetcode::leetcode947;
fn main() {
    let example = vec![vec![0,0], vec![0,1], vec![1,0], vec![1,2], vec![2,1], vec![2,2]];
    let ans = leetcode947::remove_stones(example);
    println!("{}", ans);
}
