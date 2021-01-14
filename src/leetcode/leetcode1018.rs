pub fn prefixes_div_by5(a: Vec<i32>) -> Vec<bool> {
        let mut prefix = 0;
        let mut answer = Vec::new();
        for bit in a {
            prefix = (prefix*2 + bit) % 5;
            answer.push(prefix == 0);
        }
        answer
    }