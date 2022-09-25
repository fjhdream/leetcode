pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
    let distance = manhattan_distance(vec![0,0], target.clone());
    for ghost in ghosts {
        let ghost_dist = manhattan_distance(ghost, target.clone());
        if ghost_dist <= distance {
            return false;
        }
    }
    return true;
}

fn manhattan_distance(point_1: Vec<i32>, point_2: Vec<i32>) -> usize {
    return (point_1[0] - point_2[0]).abs() as usize + (point_1[1] - point_2[1]).abs() as usize;
}

#[test]
fn test_example() {
    let ghosts = vec![vec![1, 0], vec![0, 3]];
    let target = vec![0, 1];
    let ans = escape_ghosts(ghosts, target);
    assert_eq!(ans, true);
}