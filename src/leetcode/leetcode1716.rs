pub fn total_money(n: i32) -> i32 {
    let mut weekday = n / 7;
    let first_week = (1 + 7) * 7 / 2;
    let last_week = first_week + 7 * (weekday - 1);
    let week_money = (first_week + last_week) * weekday / 2;

    let mut day = n % 7;
    let first_day = 1 + weekday;
    let last_day = first_day + day - 1;
    let day_money = (first_day + last_day) * day / 2;

    week_money + day_money
}

#[test]
fn test() {
    let n = 10;
    let ans = total_money(n);
    assert_eq!(ans, 37);
}
