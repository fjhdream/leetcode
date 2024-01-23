use std::iter::FromIterator;

pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
    words
        .into_iter()
        .flat_map(|word| {
            word.split(separator)
                .map(String::from)
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
        })
        .collect()
}

#[test]
fn test_split_words_by_separator() {
    assert_eq!(
        split_words_by_separator(vec!["a".to_string(), "b".to_string()], 'a'),
        vec!["b".to_string()]
    );
}
