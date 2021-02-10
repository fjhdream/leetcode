pub fn check_inclusion(s1: String, s2: String) -> bool {
    let mut cnt1 = vec![0;26];
    let mut cnt2 = vec![0;26];
    if s1.len() > s2.len() {
        return false;
    }
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    for i in 0..s1.len() {
        cnt1[s1[i] as usize- 97] += 1;
        cnt2[s2[i] as usize- 97] += 1;
    }
    if cnt1 == cnt2 {
        return true;
    }
    let k = s1.len();
    for i in k..s2.len() {
        cnt2[s2[i - k] as usize - 97] -= 1;
        cnt2[s2[i] as usize - 97] += 1;
        if cnt1 == cnt2 { 
            return true;
        }
    }
    false
}