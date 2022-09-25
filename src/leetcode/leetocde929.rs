use std::collections::HashSet;

pub fn num_unique_emails(emails: Vec<String>) -> i32 {
    let mut map = HashSet::new();
    for email in emails.iter() {
        let mut split_strs = email.split('@').collect::<Vec<_>>();
        let mut local_name = split_strs[0].split('+').next().unwrap().replace(".", "");
        let mut domain_name = split_strs[1];
        let mut res = local_name + "@" + domain_name;
        map.insert(res);
    }
    map.len() as i32
}

#[test]
fn test() {
    let ans = num_unique_emails(vec![
        "test.email+alex@leetcode.com".to_string(),
        "test.e.mail+bob.cathy@leetcode.com".to_string(),
        "testemail+david@lee.tcode.com".to_string(),
    ]);
    assert_eq!(ans, 2);
}

#[test]
fn test2() {
    let ans = num_unique_emails(vec![
        "a@leetcode.com".to_string(),
        "b@leetcode.com".to_string(),
        "c@leetcode.com".to_string(),
    ]);
    assert_eq!(ans, 3);
}
