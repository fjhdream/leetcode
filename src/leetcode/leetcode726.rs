use std::collections::BTreeMap;

pub fn count_of_atoms(formula: String) -> String {
    let mut counter = BTreeMap::new();
    let mut cnt = 1;
    let mut cnt_stack = vec![];
    let mut time = 1;
    let mut name: &str;
    let mut i = formula.len() - 1;
    let s = formula.as_bytes();
    loop {
        let c = s[i];
        if c.is_ascii_digit() {
            let mut j = i;
            while i >= 1 && s[i-1].is_ascii_digit() {
                i -= 1;
            }
            cnt = formula[i..=j].parse().unwrap();
        } else if c == b')' {
            time *= cnt;
            cnt_stack.push(cnt);
            cnt = 1;
        } else if c == b'(' {
            time /= cnt_stack.pop().unwrap();
        } else {
            let j = i;
            while i >= 0 && s[i].is_ascii_lowercase() {
                i -= 1;
            }
            name = &formula[i..=j];
            *counter.entry(name).or_insert(0) += time * cnt;
            cnt = 1;
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }
    counter
        .into_iter()
        .map(|(name, cnt)| {
            if cnt == 1 {
                name.to_owned()
            } else {
                name.to_owned() + &cnt.to_string()
            }
        })
        .collect()
}