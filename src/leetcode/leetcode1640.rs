use std::collections::HashMap;

pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
    let mut map = HashMap::new();
    for (i, piece) in pieces.iter().enumerate() {
        map.insert(piece[0], i);
    }
    let mut idx = 0;
    while idx < arr.len() {
        if !map.contains_key(&arr[idx]) {
            return false;
        }
        let piece_idx = *map.get(&arr[idx]).unwrap();
        for &i in pieces[piece_idx].iter() {
            if i == arr[idx] {
                idx += 1;
            } else {
                return false;
            }
        }
    }
    return true;
}
