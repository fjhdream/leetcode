pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
    let mut num = [0; 100];  
    let mut ans = 0;
    for domino in dominoes {
        let mut pair = 0;
        if domino[0] > domino[1] {
            pair = domino[1]*10 + domino[0];
        } else {
            pair = domino[0]*10 + domino[1];
        }
        ans += num[pair as usize];
        num[pair as usize] += 1;
    }
    ans
}