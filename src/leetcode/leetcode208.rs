#[derive(Default)]
struct Trie {
    root: Node
}

#[derive(Default)]
struct Node {
    path: [Option<Box<Node>>; 26],
    num: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self::default()
    }

    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for c in word.as_bytes() {
            let idx = (c - b'a') as usize;
            let next = &mut node.path[idx];
            node = next.get_or_insert_with(Box::<Node>::default);
        }
        node.num += 1;
    }

    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        self.get_node(&word).map_or(false, |w| w.num != 0)
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        self.get_node(&prefix).is_some()
    }

    fn get_node(&self, s: &str) -> Option<&Node> {
        let mut node = &self.root;
        for c in s.as_bytes() {
            let idx = (c - b'a') as usize;
            match &node.path[idx] {
                Some(next) => node = next.as_ref(),
                None => return None,
            }
        }
        Some(node)
    }
}