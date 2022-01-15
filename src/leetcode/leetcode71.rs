pub fn simplify_path(path: String) -> String {
    let mut queue = Vec::new();
    path.split("/").for_each(|letter| match letter {
        "." | "" => (),
        ".." => {
            queue.pop();
        }
        _ => queue.push(letter),
    });

    "/".to_string() + &queue.join("/")
}

#[test]
fn test() {
    let ans = simplify_path(String::from("/a/./b/../../c/"));
    assert_eq!(ans, "/c");
}
