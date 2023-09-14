use std::collections::HashSet;

pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
    let mut set = HashSet::new();
    for queen in queens {
        set.insert(queen[0] * 8 + queen[1]);
    }
    let mut res = vec![];
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let mut kx = king[0] + dx;
            let mut ky = king[1] + dy;
            while (0..8).contains(&kx) && (0..8).contains(&ky) {
                let pos = kx * 8 + ky;
                if set.contains(&pos) {
                    res.push(vec![kx, ky]);
                    break;
                }
                kx += dx;
                ky += dy;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let ans = queens_attackthe_king(
            vec![
                vec![0, 1],
                vec![1, 0],
                vec![4, 0],
                vec![0, 4],
                vec![3, 3],
                vec![2, 4],
            ],
            vec![0, 0],
        );
        assert_eq!(ans, vec![vec![0, 1], vec![1, 0], vec![3, 3]]);
    }
}
