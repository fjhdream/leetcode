pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
    if tomato_slices % 2 != 0
        || tomato_slices < cheese_slices * 2
        || cheese_slices * 4 < tomato_slices
    {
        return vec![];
    }
    vec![
        tomato_slices / 2 - cheese_slices,
        cheese_slices * 2 - tomato_slices / 2,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(num_of_burgers(16, 7), vec![1, 6]);
    }
}
