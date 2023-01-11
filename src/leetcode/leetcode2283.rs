pub fn digit_count(num: String) -> bool {
    let mut origin_map = vec![0; 10];
    let mut map = vec![0; 10];
    let chars = num.chars().collect::<Vec<_>>();
    chars
        .iter()
        .enumerate()
        .for_each(|(idx, &val)| origin_map[idx] = val.to_digit(10).unwrap() as i32);
    chars
        .iter()
        .for_each(|val| map[val.to_digit(10).unwrap() as usize] += 1);
    origin_map.eq(&map)
}
