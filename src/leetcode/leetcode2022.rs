pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
    if original.len() != (m * n) as usize {
        return vec![];
    }
    original.chunks(n as usize).map(|chunck| chunck.to_vec()).collect()
}