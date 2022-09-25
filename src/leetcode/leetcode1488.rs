use std::{collections::HashMap, vec};

pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
    let n = rains.len();
    let mut ans = vec![-1; n];
    let mut full = HashMap::new();
    let mut free_days = Vec::new();
    for (day, pool) in rains.iter().enumerate() {
        //检查是否是晴天
        if *pool == 0 {
            free_days.push(day);
        } else {
            //查看当前湖泊有没有满
            if !full.contains_key(pool) {
                full.insert(*pool, day);
            } else {
                //找到上次下雨的天,去检查有没有晴天能够抽水
                let pre_rain_day = full.get(&pool).unwrap();
                // 弹出能够抽水的晴天,  比最近的天, 小贪心
                match free_days.binary_search(pre_rain_day) {
                    Ok(_) => {
                        return vec![];
                    }
                    Err(day_idx) => {
                        if day_idx == free_days.len() {
                            return vec![];
                        } else {
                            let free_day = free_days[day_idx];
                            if *pre_rain_day > free_day {
                                return vec![];
                            } else {
                                ans[free_day] = *pool;
                                free_days.remove(day_idx);
                                full.insert(*pool, day);
                            }
                        }
                    }
                }
            }
        }
    }
    for day in free_days {
        ans[day] = 1;
    }
    ans
}

#[test]
fn test() {
    let ans = avoid_flood(vec![1, 2, 0, 0, 2, 1]);
    assert_eq!(ans, vec![-1, -1, 2, 1, -1, -1]);
}

#[test]
fn test2() {
    let ans = avoid_flood(vec![1, 2, 3, 4]);
    assert_eq!(ans, vec![-1, -1, -1, -1]);
}

#[test]
fn test3() {
    let ans = avoid_flood(vec![1, 2, 0, 1, 2]);
    assert_eq!(ans, vec![]);
}

#[test]
fn test4() {
    let ans = avoid_flood(vec![69, 0, 0, 0, 69]);
    assert_eq!(ans, vec![-1, 69, 1, 1, -1]);
}

#[test]
fn test5() {
    let ans = avoid_flood(vec![10, 20, 20]);
    assert_eq!(ans, vec![]);
}
