// see answer https://stackoverflow.com/questions/14053885/integer-partition-algorithm-and-recursion/27051927
fn partitions1(n: u32) -> u32 {
    let mut ways = vec![0; n as usize + 1];
    ways[0] = 1;
    for i in 1..=n {
        for j in i..=n {
            ways[j as usize] += ways[j as usize - i as usize];
        }
    }
    ways[n as usize]
}

fn partitions(n: u32) -> u32 {
    let mut ways = vec![vec![0; n as usize + 1]; n as usize + 1];
    ways[0][0] = 1;
    for i in 1..=n {
        for j in 0..=n {
            ways[i as usize][j as usize] = ways[i as usize - 1][j as usize];
            if j >= i {
                ways[i as usize][j as usize] += ways[i as usize][j as usize - i as usize];
            }
        }
    }
    ways[n as usize][n as usize]
}

#[cfg(test)]
mod tests {
    use super::partitions;

    #[test]
    fn basic_test_01() {
        assert_eq!(partitions(1), 1);
    }

    #[test]
    fn basic_test_05() {
        assert_eq!(partitions(5), 7);
    }

    #[test]
    fn basic_test_10() {
        assert_eq!(partitions(10), 42);
    }

    #[test]
    fn basic_test_25() {
        assert_eq!(partitions(25), 1958);
    }
}
