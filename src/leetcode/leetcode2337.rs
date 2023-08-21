pub fn can_change(start: String, target: String) -> bool {
    if start.replace('_', "") != target.replace('_', "") {
        return false;
    }
    let start_chs = start.chars().collect::<Vec<_>>();
    let target_chs = target.chars().collect::<Vec<_>>();
    let mut i = 0;
    let mut j = 0;
    loop {
        while i < start_chs.len() && start_chs[i] == '_' {
            i += 1;
        }
        while j < target_chs.len() && target_chs[j] == '_' {
            j += 1;
        }
        if i == start_chs.len() || j == target_chs.len() {
            break;
        }
        if i == start_chs.len() || j == target_chs.len() || start_chs[i] != target_chs[j] {
            return false;
        }
        if (start_chs[i] == 'L' && i < j) || (start_chs[i] == 'R' && i > j) {
            return false;
        }
        i += 1;
        j += 1;
    }
    return true;
}

#[test]
fn test() {
    let start = "_L__R__R_".to_string();
    let target = "L______RR".to_string();
    let res = can_change(start, target);
    assert_eq!(res, true);
}

#[test]
fn test2() {
    let start = "R_L_".to_string();
    let target = "__LR".to_string();
    let res = can_change(start, target);
    assert_eq!(res, false);
}

#[test]
fn test3() {
    let start = "_R".to_string();
    let target = "R_".to_string();
    let res = can_change(start, target);
    assert_eq!(res, false);
}
