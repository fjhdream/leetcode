pub fn minimum_removal(mut beans: Vec<i32>) -> i64 {
    beans.sort();
    let sum = beans.iter().map(|i| *i as i128).sum::<i128>();
    let mut res = sum;
    for i in 0..beans.len() {
        res = res.min(sum - beans[i] as i128 * (beans.len() as i128 - i as i128));
    }
    res as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(minimum_removal(vec![6, 1, 4, 5]), 4);
    }
}
