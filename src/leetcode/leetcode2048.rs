pub fn next_beautiful_number(n: i32) -> i32 {
    let mut num = n + 1;
    while !is_balance(num) {
        num += 1;
    }
    return num;

}

fn is_balance(n: i32) -> bool {
    let mut cnt = vec![i32::MAX; 10];
    let mut num = n;
    while num != 0 {
        let mod_num = (num % 10) as usize;
        if cnt[mod_num] == i32::MAX {
            cnt[mod_num] = 1;
        } else {
            cnt[mod_num] += 1;
        }
        num = num / 10;
    }
    
    for (idx, val) in cnt.iter().enumerate() {
        if *val != i32::MAX && idx as i32 != *val {
            return false;
        }
    }
    true
}


#[test]
fn test() {
    let ans = next_beautiful_number(1);
    assert_eq!(ans, 22);
}

#[test]
fn test2() {
    let ans = next_beautiful_number(1000);
    assert_eq!(ans, 1333);
}

#[test]
fn test3() {
    let ans = next_beautiful_number(3000);
    assert_eq!(ans, 3133);
}