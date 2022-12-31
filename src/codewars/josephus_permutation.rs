fn josephus<T: Clone + Copy>(xs: Vec<T>, k: usize) -> Vec<T> {
    let mut res = Vec::new();
    let mut xs = xs;
    let mut i = 0;
    while !xs.is_empty() {
        i = (i + k - 1) % xs.len();
        res.push(xs.remove(i));
    }
    res
}
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod josephus {
    use super::josephus;

    #[test]
    fn test_works_with_integers() {
        assert_eq!(
            josephus(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 1),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        );
        assert_eq!(
            josephus(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 2),
            vec![2, 4, 6, 8, 10, 3, 7, 1, 9, 5]
        );
    }
    #[test]
    fn test_works_with_strings() {
        assert_eq!(
            josephus("CodeWars".chars().collect::<Vec<char>>(), 4),
            "esWoCdra".chars().collect::<Vec<char>>()
        );
    }
}
