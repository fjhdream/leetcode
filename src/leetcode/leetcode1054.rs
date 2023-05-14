use std::collections::HashMap;

pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
    let mut count_map = HashMap::new();
    for &barcode in barcodes.iter() {
        let count = count_map.entry(barcode).or_insert(0);
        *count += 1;
    }
    let mut count_vec: Vec<(i32, i32)> = count_map.into_iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(&a.1));
    let mut res = vec![0; barcodes.len()];
    let mut index = 0;
    for (barcode, count) in count_vec {
        for _ in 0..count {
            res[index] = barcode;
            index += 2;
            if index >= barcodes.len() {
                index = 1;
            }
        }
    }
    res
}

#[test]
fn test() {
    let barcodes = vec![1, 1, 1, 2, 2, 2];
    let res = vec![1, 2, 1, 2, 1, 2];
    assert_eq!(rearrange_barcodes(barcodes), res);
    let barcodes = vec![1, 1, 1, 1, 2, 2, 3, 3];
    let res = vec![1, 2, 1, 2, 1, 3, 1, 3];
    assert_eq!(rearrange_barcodes(barcodes), res);
}