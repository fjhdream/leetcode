pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
    let mut pressed_times = vec![release_times[0]];
    release_times
        .windows(2)
        .for_each(|times| pressed_times.push(times[1] - times[0]));
    let key = keys_pressed.chars().collect::<Vec<_>>();
    let time_key_pairs = pressed_times.iter().zip(key.iter());
    *time_key_pairs.max().unwrap().1
}

#[test]
fn test() {
    let release_times = vec![9, 29, 49, 50];
    let keys_pressed = String::from("cbcd");
    let ans = slowest_key(release_times, keys_pressed);
    assert_eq!(ans, 'c');
}
