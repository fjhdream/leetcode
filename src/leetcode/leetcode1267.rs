use std::collections::HashMap;

pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
    let mut row = HashMap::new();
    let mut col = HashMap::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 1 {
                *row.entry(i).or_insert(0) += 1;
                *col.entry(j).or_insert(0) += 1;
            }
        }
    }
    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 1 && (row[&i] > 1 || col[&j] > 1) {
                ans += 1;
            }
        }
    }
    ans
}

mod test {
    use super::*;

    #[test]
    fn test_count_servers() {
        assert_eq!(count_servers(vec![vec![1, 0], vec![0, 1]]), 0);
        assert_eq!(count_servers(vec![vec![1, 0], vec![1, 1]]), 3);
        assert_eq!(
            count_servers(vec![
                vec![1, 1, 0, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 0, 1]
            ]),
            4
        );
    }
}
