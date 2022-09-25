pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut arr: Vec<(usize, i32)> = mat.into_iter().enumerate().map(|(i, row)| {
        let (mut l, mut r) = (0, row.len());
        while l < r {
            let m = l + (r - l) / 2;
            if row[m] == 1 {
                l = m + 1;
            } else {
                r = m;
            }
        }
        (l, i as i32)
    }).collect();
    arr.sort();
    arr.truncate(k as usize);
    arr.into_iter().map(|(_, i)| i).collect()
}