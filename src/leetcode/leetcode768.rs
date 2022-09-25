pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
    let mut stack = Vec::new();
    for num in arr {
        if stack.is_empty() || &num >= stack.last().unwrap() {
            stack.push(num);
        } else {
            let mx = stack.pop().unwrap();
            while !stack.is_empty() && stack.last().unwrap() > &num {
                stack.pop();
            }
            stack.push(mx);
        }
    }
    stack.len() as i32
}