pub fn check_record(s: String) -> bool {
    let char_list = s.chars().collect::<Vec<_>>();
    let mut absents = 0;
    let mut lates = 0;
    for char in char_list {
        if char == 'A' {
            absents += 1;
            if absents >= 2 {
                return false;
            }
        }
        if char == 'L' {
            lates += 1;
            if lates >= 3 {
                return false;
            }
        } else {
            lates = 0;
        }
    }
    return true;
}