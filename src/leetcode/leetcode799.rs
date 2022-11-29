pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
    let mut row = vec![poured as f64];
    if query_row == 0 {
        return 1.0_f64.min(row[query_glass as usize]);
    }
    for i in 1..=query_row {
        let mut next_row = vec![0f64; i as usize + 1];
        for j in 0..i as usize {
            let volume = row[j];
            if volume > 1.0 {
                next_row[j] = next_row[j] + (volume - 1.0) / 2.0;
                next_row[j + 1] = next_row[j + 1] + (volume - 1.0) / 2.0;
            }
        }
        row = next_row;
    }
    return 1.0_f64.min(row[query_glass as usize]);
}

#[test]
fn test() {
    let ans = champagne_tower(1, 1, 1);
    assert_eq!(ans, 0.0);
}

#[test]
fn test2() {
    let ans = champagne_tower(2, 1, 1);
    assert_eq!(ans, 0.5);
}

#[test]
fn test3() {
    let ans = champagne_tower(100000009, 33, 17);
    assert_eq!(ans, 1.0);
}

#[test]
fn test4() {
    let ans = champagne_tower(0, 0, 0);
    assert_eq!(ans, 0.0);
}

#[test]
fn test5() {
    let ans = champagne_tower(25, 6, 1);
    assert_eq!(ans, 0.1875);
}
