pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
    let m = target.len();
    let mut memo = vec![-1; 1 << m];
    memo[0] = 0;
    let target = target.chars().collect::<Vec<_>>();
    let res = dp(&stickers, &target, &mut memo, (1 << m) - 1);
    if res > m as i32 {
        -1
    } else {
        res
    }
}

fn dp(stickers: &Vec<String>, target: &Vec<char>, memo: &mut Vec<i32>, mask: i32) -> i32 {
    let m = target.len();
    if memo[mask as usize] != -1 {
        return memo[mask as usize];
    }
    let mut res = m as i32 + 1;
    for sticker in stickers {
        let mut tmp_mask = mask;
        let mut cnt = vec![0; 26];
        sticker.chars().for_each(|c| cnt[c as usize - 'a' as usize] += 1);
        for i in 0..m {
            let ch = target[i];
            if (mask >> i) & 1 == 1 && cnt[ch as usize - 'a' as usize] > 0 {
                tmp_mask ^= 1 << i;
                cnt[ch as usize - 'a' as usize] -= 1;
            }
        }
        if tmp_mask < mask {
            res = res.min(dp(stickers, target, memo, tmp_mask) + 1);
        }
    }
    memo[mask as usize] = res;
    memo[mask as usize]
}


#[test]
fn test() {
    let res = min_stickers(vec!["with".to_string(),"example".to_string(),"science".to_string()], "thehat".to_string());
    assert_eq!(res, 3);
}