use leetcode::leetcode::*;
fn main() {
    let example = vec![ vec![0,1], vec![0,2], vec![1,2]];
    let ans = leetcode1319::make_connected(4, example);
    println!("{}", ans);
}
