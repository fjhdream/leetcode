use std::collections::HashMap;

pub fn check_valid_grid(grid: Vec<Vec<i32>>) -> bool {
    if grid[0][0] != 0 {
        return false;
    }
    let mut map = HashMap::new();
    let row = grid.len();
    let col = grid[0].len();
    for x in 0..row {
        for y in 0..col {
            map.insert(grid[x][y], (x, y));
        }
    }
    for i in 0..(row * col) as i32 - 1 {
        if !is_valid(*map.get(&i).unwrap(), *map.get(&(i + 1)).unwrap()) {
            return false;
        }
    }
    return true;
}

fn is_valid((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> bool {
    let x_diff = x1.max(x2) - x1.min(x2);
    let y_diff = y1.max(y2) - y1.min(y2);
    if x_diff == 1 && y_diff == 2 {
        return true;
    }
    if x_diff == 2 && y_diff == 1 {
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use crate::leetcode::leetcode2596::check_valid_grid;

    #[test]
    fn test() {
        let ans = check_valid_grid(vec![
            vec![0, 11, 16, 5, 20],
            vec![17, 4, 19, 10, 15],
            vec![12, 1, 8, 21, 6],
            vec![3, 18, 23, 14, 9],
            vec![24, 13, 2, 7, 22],
        ]);
        assert_eq!(ans, true)
    }

    #[test]
    fn test2() {
        let ans = check_valid_grid(vec![vec![0, 3, 6], vec![5, 8, 1], vec![2, 7, 4]]);
        assert_eq!(ans, false)
    }
}
