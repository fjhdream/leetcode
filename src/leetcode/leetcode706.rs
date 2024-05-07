pub fn shortest_completing_word(license_plate: String, mut words: Vec<String>) -> String {
    let mut char_arr = vec![0; 26];
    for ele in license_plate.chars() {
        if ele.is_ascii_alphabetic() {
            let idx = (ele.to_ascii_lowercase() as u32 - 'a' as u32) as usize;
            char_arr[idx] += 1;
        }
    }
    let mut match_arr = vec![0; 26];
    words.sort_by(|a, b| a.len().cmp(&b.len()));
    for ele in words {
        ele.chars()
            .for_each(|c| match_arr[(c.to_ascii_lowercase() as u32 - 'a' as u32) as usize] += 1);
        for (idx, num) in char_arr.iter().enumerate() {
            if *num > match_arr[idx] {
                break;
            }
            if idx == 25 && *num <= match_arr[idx] {
                return ele;
            }
        }
        match_arr = vec![0; 26];
    }
    return "".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let license_plate = "1s3 PSt".to_string();
        let words = vec![
            "step".to_string(),
            "steps".to_string(),
            "stripe".to_string(),
            "stepple".to_string(),
        ];
        assert_eq!(
            shortest_completing_word(license_plate, words),
            "steps".to_string()
        );
    }

    #[test]
    fn test_2() {
        let license_plate = "1s3 456".to_string();
        let words = vec![
            "looks".to_string(),
            "pest".to_string(),
            "stew".to_string(),
            "show".to_string(),
        ];
        assert_eq!(
            shortest_completing_word(license_plate, words),
            "pest".to_string()
        );
    }
}
