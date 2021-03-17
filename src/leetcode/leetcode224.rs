macro_rules! map {
    ($($key: expr => $val: expr), *) => {{
        let mut dict = std::collections::HashMap::new();
        $( dict.insert($key, $val); )*
        dict
    }};
}

pub fn calculate(s: String) -> i32 {
    let mut nums = vec![];
    let mut ops = vec![];
    let mut num = None;
    let pri = map!(b'-' => 1, b'+' => 1, b'*' => 2, b'/' => 2, b'('=> 10, b')' => 0);

    for ch in s.bytes().chain(")".bytes()).filter(|x| *x != b' ') {
        if ch.is_ascii_digit() {
            let n = num.get_or_insert(0);
            *n = 10 * *n + (ch - b'0') as i32;
            continue;
        }
        num.take().map(|x| nums.push(x));
        while ops
            .last()
            .map_or(false, |x| *x != b'(' && pri[x] >= pri[&ch])
        {
            ops.pop().map(|x| _calc(&mut nums, x));
        }

        if ch != b')' {
            ops.push(ch);
        } else {
            ops.pop();
        }
    }
    nums.pop().unwrap_or(0)
}

fn _calc(nums: &mut Vec<i32>, op: u8) {
    let r = nums.pop().unwrap_or(0);
    let l = nums.pop().unwrap_or(0);
    nums.push(match op {
        b'+' => l + r,
        b'-' => l - r,
        b'*' => l * r,
        b'/' => l / r,
        _ => panic!(),
    });
}

#[test]
pub fn test_example() {
    let s = "(1+(4+5+2)-3)+(6+8)";
    let ans = calculate(s.to_string());
}