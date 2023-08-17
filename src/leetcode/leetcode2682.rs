pub fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
    let mut visited = vec![false; n as usize];
    let mut res = Vec::new();
    let mut next = 0;
    let mut round = 1;
    while visited[next] == false {
        visited[next] = true;
        let step = round * k;
        next = (next + step as usize) % n as usize;
        round += 1;
    }
    visited.iter().enumerate().for_each(|(i, &v)| {
        if v == false {
            res.push(i as i32 + 1);
        }
    });
    res
}

#[test]
fn test_circular_game_losers() {
    let n = 5;
    let k = 2;
    let res = circular_game_losers(n, k);
    println!("{:?}", res);
}

#[test]
fn test_circular_game_losers2() {
    let n = 4;
    let k = 4;
    let res = circular_game_losers(n, k);
    println!("{:?}", res);
}
