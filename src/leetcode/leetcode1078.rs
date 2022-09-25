pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
    let mut ret = Vec::new();
    let split_arr: Vec<&str> = text.split_whitespace().collect();
    for i in 0..split_arr.len() - 1 {
        if split_arr[i] == first && split_arr[i + 1] == second && i + 2 < split_arr.len() {
            ret.push(split_arr[i + 2].to_string());
        }
    }
    ret
}

#[test]
fn test() {
    let text = "alice is a good girl she is a good student".to_string();
    let first = "a".to_string();
    let second = "good".to_string();
    let ans = find_ocurrences(text, first, second);
    assert_eq!(ans, vec![String::from("girl"), String::from("student")]);
}