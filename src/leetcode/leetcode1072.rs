use std::collections::HashMap;

pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
    let mut map = HashMap::new();
    matrix
        .into_iter()
        .map(|mut arr| -> String {
            arr = if arr[0] == 0 {
                arr.into_iter().map(|num| 1 - num).collect::<Vec<i32>>()
            } else {
                arr
            };
            arr.into_iter()
                .map(|num| char::from_u32(num as u32).unwrap())
                .collect::<String>()
        })
        .for_each(|s| *map.entry(s).or_insert(0) += 1);
    map.into_iter().map(|(_, v)| v).max().unwrap()
}
