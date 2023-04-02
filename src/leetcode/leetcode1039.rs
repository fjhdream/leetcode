pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
    let n = values.len();
    let mut cache = vec![vec![0; n]; n];
    return min_score(&mut cache, &values, 0, n - 1);
}

fn min_score(cache: &mut Vec<Vec<i32>>, values: &Vec<i32>, i: usize, j: usize) -> i32 {
    if i + 1 == j {
        return 0;
    }
    if cache[i][j] != 0 {
        return cache[i][j];
    }
    let mut min = i32::MAX;
    for k in i + 1..j {
        let left = min_score(cache, values, i, k);
        let right = min_score(cache, values, k, j);
        let sum = left + right + values[i] * values[k] * values[j];
        if sum < min {
            min = sum;
        }
    }
    cache[i][j] = min;
    min
}

pub fn min_score_triangulation2(values: Vec<i32>) -> i32 {
    let n = values.len();
    let mut dp = vec![vec![0; n]; n];
    for i in (0..=n - 3).rev() {
        for j in i + 2..n {
            let mut min = i32::MAX;
            for k in i + 1..j {
                let sum = dp[i][k] + dp[k][j] + values[i] * values[k] * values[j];
                if sum < min {
                    min = sum;
                }
            }
            dp[i][j] = min;
        }
    }
    return dp[0][n - 1];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1039() {
        assert_eq!(min_score_triangulation(vec![1, 2, 3]), 6);
        assert_eq!(min_score_triangulation(vec![3, 7, 4, 5]), 144);
        assert_eq!(min_score_triangulation(vec![1, 3, 1, 4, 1, 5]), 13);
    }

    #[test]
    fn test_10392() {
        assert_eq!(min_score_triangulation2(vec![1, 2, 3]), 6);
        assert_eq!(min_score_triangulation2(vec![3, 7, 4, 5]), 144);
        assert_eq!(min_score_triangulation2(vec![1, 3, 1, 4, 1, 5]), 13);
    }
}
