pub fn parse_bool_expr(expression: String) -> bool {
    let mut stack = Vec::new();
    for ch in expression.chars() {
        match ch {
            ',' => {
                continue;
            }
            ')' => {
                let (mut t, mut f) = (0, 0);
                while let Some(val) = stack.pop() {
                    if val == '(' {
                        break;
                    }
                    if val == 't' {
                        t += 1;
                    } else {
                        f += 1;
                    }
                }
                if let Some(op) = stack.pop() {
                    match op {
                        '!' => {
                            stack.push(if f == 1 { 't' } else { 'f' });
                        }
                        '&' => {
                            stack.push(if f == 0 { 't' } else { 'f' });
                        }
                        '|' => {
                            stack.push(if t > 0 { 't' } else { 'f' });
                        }
                        _ => {}
                    }
                }
            }
            other => stack.push(other),
        }
    }
    stack.pop().unwrap() == 't'
}

#[test]
fn test() {
    let ans = parse_bool_expr("&(|(f))".to_string());
    assert!(!ans);
}
