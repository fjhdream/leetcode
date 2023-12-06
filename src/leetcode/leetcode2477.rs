pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
    let mut g = vec![vec![]; roads.len() + 1];
    for e in &roads {
        let x = e[0] as usize;
        let y = e[1] as usize;
        g[x].push(y);
        g[y].push(x);
    }
    let mut ans = 0i64;
    dfs(&g, seats, 0, 0, &mut ans);
    ans
}

fn dfs(g: &Vec<Vec<usize>>, seats: i32, x: usize, fa: usize, ans: &mut i64) -> i32 {
    let mut size = 1;
    for &y in &g[x] {
        if y != fa {
            size += dfs(g, seats, y, x, ans);
        }
    }
    if x != 0 {
        *ans += ((size - 1) / seats + 1) as i64;
    }
    size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_fuel_cost() {
        assert_eq!(
            minimum_fuel_cost(vec![vec![0, 1], vec![0, 2], vec![0, 3]], 5),
            3
        );
    }

    #[test]
    fn test_minimum_fuel_cost_1() {
        assert_eq!(
            minimum_fuel_cost(
                vec![
                    vec![3, 1],
                    vec![3, 2],
                    vec![1, 0],
                    vec![0, 4],
                    vec![0, 5],
                    vec![4, 6]
                ],
                2
            ),
            7
        );
    }
}
