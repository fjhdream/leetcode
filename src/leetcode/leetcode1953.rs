pub fn number_of_weeks(milestones: Vec<i32>) -> i64 {
    let mut longest: i64 = 0;
    let mut rest: i64 = 0;
    for m in milestones {
        longest = longest.max(m as i64);
        rest += m as i64;
    }
    rest -= longest;
    if longest > rest + 1 {
        rest * 2 + 1
    } else {
        rest + longest
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_of_weeks() {
        assert_eq!(number_of_weeks(vec![1, 2, 3]), 6);
    }

    #[test]
    fn test_number_of_weeks_2() {
        assert_eq!(number_of_weeks(vec![5, 2, 1]), 7);
    }
}
