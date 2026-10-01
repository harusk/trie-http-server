mod trie;

use trie::Trie::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_trie_and_adds_root_paths() {
        let mut new_trie = Trie::initialize();
        new_trie.add_root_path(String::from("echo"), String::from("GET"));
        new_trie.add_root_path(String::from("hello"), String::from("GET"));
        new_trie.add_root_path(String::from("echo"), String::from("GET"));

        for e in new_trie.root.node {
            dbg!(e);
        }
    }
}
