pub fn rotated_digits(n: i32) -> i32 {
    let check = [0, 0, 1, -1, -1, 1, 1, -1, 0, 1];
    let mut res = 0;
    for mut i in 1..=n {
        let (mut valid, mut diff) = (true, false);
        while i > 0 && valid {
            let num = i % 10;
            if check[num as usize] == -1 {
                valid = false;
            } else if check[num as usize] == 1 {
                diff = true;
            }
            i /= 10;
        }
        if valid && diff {
            res += 1;
        }
    }
    res
}