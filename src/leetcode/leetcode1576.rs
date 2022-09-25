pub fn modify_string(s: String) -> String {
    let mut vs = s.chars().collect::<Vec<_>>();
    for i in 0..vs.len() {
        if vs[i] != '?' {
            continue;
        }
        let mut nc = 'a';
        while (i > 0 && vs[i-1] == nc) || (i < vs.len() - 1 && vs[i+1] == nc) {
            nc = (nc as u8 + 1) as char;
        }
        vs[i] = nc;
    }
    vs.iter().collect()
}

#[test]
fn test() {
    let s = String::from("??yw?ipkj?");
    let ans = modify_string(s);
    println!("{:?}", ans)
}