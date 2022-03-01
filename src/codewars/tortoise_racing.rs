fn race(v1: i32, v2: i32, g: i32) -> Option<Vec<i32>> {
    if v1 >= v2 {return None;}
    let time = g * 3600 / (v2 - v1);
    let (hours, mins, seconds) = (time / 3600, (time % 3600) / 60, time % 60);
    let ans = vec![hours, mins, seconds];
    Some(ans)
}

  #[test]
fn basic_tests() {
    assert_eq!(race(720, 850, 70), Some(vec![0, 32, 18]));
    assert_eq!(race(80, 100, 40), Some(vec![2, 0, 0]));
    assert_eq!(race(80, 91, 37), Some(vec![3, 21, 49]));
    assert_eq!(race(820, 81, 550), None);
}