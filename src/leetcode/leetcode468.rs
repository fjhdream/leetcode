pub fn valid_ip_address(query_ip: String) -> String {
    if query_ip.contains('.') && check_ipv4(&query_ip) {
        return "IPv4".to_string();
    }
    if query_ip.contains(':') && check_ipv6(&query_ip) {
        return "IPv6".to_string();
    }
    return "Neither".to_string();
}

fn check_ipv4(ip: &str) -> bool {
    let n = ip.len();
    let mut count = 0;
    let ip_chars = ip.chars().collect::<Vec<_>>();
    let mut i = 0usize;
    while i < n && count <= 3 {
        let mut j = i;
        let mut x = 0;
        while j < n && ip_chars[j].is_ascii_digit() && x <= 255 {
            x = x * 10 + ip_chars[j].to_digit(10).unwrap() as i32;
            j += 1;
        }
        if j == i || j - i > 3 || x > 255 || (j - i > 1 && ip_chars[i] == '0') {
            return false;
        }
        i = j + 1;
        if j == n {
            continue;
        }
        if ip_chars[j] != '.' {
            return false;
        }
        count += 1;
    }
    return count == 3 && ip_chars[0] != '.' && ip_chars[n - 1] != '.';
}

fn check_ipv6(ip: &str) -> bool {
    let n = ip.len();
    let mut count = 0;
    let ip_chars = ip.chars().collect::<Vec<_>>();
    let mut i = 0usize;
    while i < n && count <= 7 {
        let mut j = i;
        while j < n
            && (ip_chars[j].is_ascii_digit()
                || (ip_chars[j] >= 'a' && ip_chars[j] <= 'f')
                || (ip_chars[j] >= 'A' && ip_chars[j] <= 'F'))
        {
            j += 1;
        }
        if j == i || j - i > 4 {
            return false;
        }
        i = j + 1;
        if j == n {
            continue;
        }
        if ip_chars[j] != ':' {
            return false;
        }
        count += 1;
    }
    return count == 7 && ip_chars[0] != ':' && ip_chars[n - 1] != ':';
}

#[test]
fn test() {
    let ans = valid_ip_address("172.168.123.123".to_string());
    assert_eq!(ans, "IPv4");
}

#[test]
fn test2() {
    let ans = valid_ip_address("2001:0db8:85a3:0:0:8A2E:0370:7334".to_string());
    assert_eq!(ans, "IPv6");
}

#[test]
fn test3() {
    let ans = valid_ip_address("256.256.256.256".to_string());
    assert_eq!(ans, "Neither");
}
