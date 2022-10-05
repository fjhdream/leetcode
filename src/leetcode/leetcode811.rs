use std::collections::HashMap;

pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
    let mut map: HashMap<String, i64> = HashMap::new();
    for cpdomain in cpdomains {
        let split_vec = cpdomain.split(' ').collect::<Vec<_>>();
        let (count, domain_str) = (split_vec[0], split_vec[1]);
        let count = count.parse::<i64>().unwrap();
        let domains = domain_str.split('.').collect::<Vec<_>>();
        let len = domains.len();
        for i in (1..len).rev() {
            let domain = domains[i..len + 1].join(".");
            *map.entry(domain).or_default() += count;
        }
    }
    let mut ans: Vec<String> = vec![];
    for (key, val) in map {
        ans.push(val.to_string() + " " + &key);
    }
    ans
}
