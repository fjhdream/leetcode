#[allow(dead_code)]
pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = Vec::new();
    for token in tokens {
        if is_operator(&token) {
            let operand2 = stack.pop().unwrap();
            let operand1 = stack.pop().unwrap();
            stack.push(calculate(operand1, token.as_str(), operand2))
        } else {
            let num = i32::from_str_radix(token.as_str(), 10).unwrap();
            stack.push(num);
        }
    }
    stack.pop().unwrap() 
}

fn is_operator(token : &String) -> bool {
    token.eq("+") || token.eq("-") || token.eq("*") || token.eq("/")
}

fn calculate(operand1: i32, operator: &str, operand2: i32) -> i32 {
    match operator {
        "+" => operand1 + operand2,
        "-" => operand1 - operand2,
        "*" => operand1 * operand2,
        "/" => operand1 / operand2,
        _ => -1
    }
}