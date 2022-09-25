use std::{collections::HashMap, vec};

fn last_digit(str1: &str, str2: &str) -> i32 {
    let mut map = HashMap::new();
    map.insert(0, vec![0]);
    map.insert(1, vec![1]);
    map.insert(2, vec![2, 4, 8, 6]);
    map.insert(3, vec![3, 9, 7, 1]);
    map.insert(4, vec![4, 6]);
    map.insert(5, vec![5]);
    map.insert(6, vec![6]);
    map.insert(7, vec![7, 9, 3, 1]);
    map.insert(8, vec![8, 4, 2, 6]);
    map.insert(9, vec![9, 1]);
    let chs = str1.chars().collect::<Vec<_>>();
    let last_num = chs.last().unwrap().to_string().parse::<i32>().unwrap();
    let chs2 = str2.chars().collect::<Vec<_>>();
    let mod_vec = map.get(&last_num).unwrap();
    let mod_num = mod_vec.len() as i32;
    if chs2.len() == 1 {
        let last_num_2 = chs2.last().unwrap().to_string().parse::<i32>().unwrap();
        if last_num_2 == 0 {
            return 1;
        }
        return mod_vec[((last_num_2 - 1) % mod_num) as usize];
    } else {
        let num_1 = chs2.last().unwrap().to_string().parse::<i32>().unwrap();
        let num_2 = chs2[chs2.len() - 2].to_string().parse::<i32>().unwrap();
        let last_num_2 = num_2 * 10 + num_1;
        return mod_vec[((last_num_2 - 1) % mod_num) as usize];
    }
}

#[test]
fn returns_expected() {
    assert_eq!(last_digit("4", "1"), 4);
    assert_eq!(last_digit("4", "2"), 6);
    assert_eq!(last_digit("9", "7"), 9);
    assert_eq!(last_digit("10", "10000000000"), 0);
    assert_eq!(last_digit("1606938044258990275541962092341162602522202993782792835301376","2037035976334486086268445688409378161051468393665936250636140449354381299763336706183397376"), 6);
    assert_eq!(
        last_digit(
            "3715290469715693021198967285016729344580685479654510946723",
            "68819615221552997273737174557165657483427362207517952651"
        ),
        7
    );
}
