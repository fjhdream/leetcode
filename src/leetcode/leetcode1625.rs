pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
    let mut min = s.clone();
    let copy = s.clone();
    let n = s.len();
    let ss = s + &copy;
    let s = ss.chars().collect::<Vec<char>>();
    let mut visited = vec![false; n];
    
    let mut idx = 0;
    while !visited[idx] {
        visited[idx] = true;
        for add_odd in 0..=9 {
            let add_even_num = if b % 2 == 0 { 0 } else { 9 };
            for add_even in 0..=add_even_num {
                let mut tmp = s[idx..idx+n].to_vec();
                for i in (1..tmp.len()).step_by(2) {
                    tmp[i] = ((tmp[i] as u8 - '0' as u8 + add_odd * a as u8) % 10 + '0' as u8) as char;
                }
                for i in (0..tmp.len()).step_by(2) {
                    tmp[i] = ((tmp[i] as u8 - '0' as u8 + add_even * a as u8) % 10 + '0' as u8) as char;
                }
                let tmp = tmp.iter().collect::<String>();
                if tmp < min {
                    min = tmp;
                }
            }
        }
        idx = (idx + b as usize) % n;
    }
    min
}


#[test]
fn test() {
    assert_eq!(find_lex_smallest_string("5525".to_string(), 9, 2), "2050".to_string());
    assert_eq!(find_lex_smallest_string("74".to_string(), 5, 1), "24".to_string());
    assert_eq!(find_lex_smallest_string("0011".to_string(), 4, 2), "0011".to_string());
    assert_eq!(find_lex_smallest_string("43987654".to_string(), 7, 3), "00553311".to_string());
    assert_eq!(find_lex_smallest_string("192804".to_string(), 8, 5), "001524".to_string());
}