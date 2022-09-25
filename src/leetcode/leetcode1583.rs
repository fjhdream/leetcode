pub fn unhappy_friends(n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut order = vec![vec![0;n];n];
    for i in 0..n {
        for j in 0..n-1 {
            order[i][preferences[i][j] as usize] = j;
        }
    }
    let mut matches = vec![0;n];
    for pair in pairs {
        let p0 = pair[0] as usize;
        let p1 = pair[1] as usize;
        matches[p0] = p1;
        matches[p1] = p0;
    }

    let mut unhappy_counts = 0;
    for x in 0..n {
        let y = matches[x];
        let index = order[x][y];
        for i in 0..index {
            let u = preferences[x][i] as usize;
            let v = matches[u];
            if order[u][x] < order[u][v] {
                unhappy_counts += 1;
                break;
            }
        }
    }
    return unhappy_counts;
}