use std::collections::HashMap;

pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
    if (max_choosable_integer + 1) * max_choosable_integer / 2 < desired_total {
        return false;
    }
    let mut memo = HashMap::new();
    return can_i_win_helper(max_choosable_integer, desired_total, 0,  0, &mut memo);
}

fn can_i_win_helper(max_choosable_integer: i32, desired_total: i32, used_numbers: i32, cur_total: i32, memo: &mut HashMap<i32, bool>) -> bool {
    if !memo.contains_key(&used_numbers) {
        let mut res = false;
        for i in 0..max_choosable_integer {
            if (1 << i) & used_numbers == 0 {
                let new_total = cur_total + i + 1;
                if new_total >= desired_total {
                    res = true;
                    break;
                }
                // 是否存在选择一个数, 使得另一方不能获胜
                if !can_i_win_helper(max_choosable_integer, desired_total, used_numbers | (1 << i), new_total, memo) {
                    res = true;
                    break;
                }
            }
        }
        memo.insert(used_numbers, res);
        res
    } else {
        memo[&used_numbers]

    }
}

#[test]
fn test() {
    let ans = can_i_win(10, 11);
    assert_eq!(ans, false);
}


#[test]
fn test2() {
    let ans = can_i_win(10, 0);
    assert_eq!(ans, true);
}