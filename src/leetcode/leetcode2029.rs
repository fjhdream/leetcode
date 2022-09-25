pub fn stone_game_ix(stones: Vec<i32>) -> bool {
    let mut cnts = [0; 3];
    stones
        .iter()
        .for_each(|stone| cnts[(stone % 3) as usize] += 1);
    if cnts[0] % 2 == 0 {
        return cnts[1] >= 1 && cnts[2] >= 1;
    }
    cnts[1] - cnts[2] > 2 || cnts[2] - cnts[1] > 2
}

#[test]
fn test() {
    let stones = vec![5, 1, 2, 4, 3];
    let ans = stone_game_ix(stones);
    assert_eq!(ans, false);
}