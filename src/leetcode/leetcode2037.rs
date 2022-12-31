pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
    seats.sort_unstable();
    students.sort_unstable();
    seats
        .into_iter()
        .zip(students.into_iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2037() {
        assert_eq!(min_moves_to_seat(vec![3, 1, 5], vec![2, 7, 4]), 4);
        assert_eq!(min_moves_to_seat(vec![4, 1, 5, 9], vec![1, 3, 2, 6]), 7);
        assert_eq!(min_moves_to_seat(vec![2, 2, 6, 6], vec![1, 3, 2, 6]), 4);
    }
}
