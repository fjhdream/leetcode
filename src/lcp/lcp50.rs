pub fn give_gem(mut gem: Vec<i32>, operations: Vec<Vec<i32>>) -> i32 {
    for op in operations {
        let (a, b) = (op[0] as usize, op[1] as usize);
        gem[b] += gem[a] / 2;
        gem[a] -= gem[a] / 2;
    }
    gem.iter().max().unwrap() - gem.iter().min().unwrap()
}

#[test]
fn test() {
    let ans = give_gem(vec![3, 1, 2], vec![vec![0, 2], vec![2, 1], vec![2, 0]]);
    assert_eq!(ans, 2);
}
