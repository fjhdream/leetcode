pub fn reverse_parentheses(s: String) -> String {
    let n = s.len();
    let mut chars : Vec<char>= s.chars().collect();
    let mut stack = Vec::new();

    for i in 0..n {
        if chars[i] == '(' {
            stack.push(i);
        } else if chars[i] == ')' {
            let start = stack.pop().unwrap();
            let end = i;
            for j in start..((start + end) / 2 + 1) {
                chars.swap(j, end - j + start);
            }
        }
    }
    let mut answer = String::new();
    for i in 0..n {
        if chars[i] != '(' && chars[i] != ')' {
            answer.push(chars[i]);
        }
    }
    answer
}

pub fn reverse_parentheses2(s: String) -> String {
    let n = s.len();
    let mut pair = vec![0 as usize; n];
    let mut stack = Vec::new();

    let mut chars : Vec<char> = s.chars().collect();
    for i in 0..n {
        if chars[i] == '(' {
            stack.push(i);
        } else if chars[i] == ')'{
            let j = stack.pop().unwrap();
            pair[i] = j;
            pair[j] = i;
        }
    }

    let mut index = 0;
    let mut step = 1;
    let mut ans = String::new();
    while index < n {
        if chars[index] == '(' || chars[index] == ')' {
            index = pair[index];
            step = -step;
        } else {
            ans.push(chars[index]);
        }
        index = (index as i32 + step) as usize;
    }

    ans
}

#[test]
pub fn test() {
    let s = "(ed(et(oc))el)";
    let ans = reverse_parentheses(String::from(s));
    let ans2 = reverse_parentheses2(String::from(s));
    println!("ans is ({})", ans);
    println!("ans2 is ({})", ans2);
    assert_eq!(ans, ans2);
}