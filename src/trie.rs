pub mod Trie {

    #[derive(Debug)]
    pub struct Entry {
        pub path: Option<String>,
        pub method: Option<String>,
        pub child: Option<Box<Node>>
    }

    // It is enough just knowing the path
    impl PartialEq for Entry {
        fn eq(&self, other: &Self) -> bool {
            self.path == other.path && self.method == other.method
        }
    }

    #[derive(Debug)]
    pub struct Node {
        pub node: Vec<Entry>
    }

    pub struct Trie {
        pub root: Node
    }

    impl Trie {
        pub fn initialize() -> Trie {
            Trie {
                root: Node {
                    node: vec![
                        Entry {
                            path: None,
                            method: None,
                            child: None
                        }
                    ]
                }
            }
        }

        fn is_root_empty(root_node: &Entry) -> bool {
            match &root_node.path {
                Some(_path) => false,
                None => true
            }
        }

        // this adds to the root path
        // need to check again how the borrow system works
        pub fn add_root_path(&mut self, path: String, method: String) {
            let root_node = &self.root.node;
            let check_root = Self::is_root_empty(&root_node[0]);

            let new_entry = Entry {
                path: Some(path),
                method: Some(method),
                child: None
            };
            // root_node should be out of scope here. No longer used
            let root = &mut self.root.node;
            if check_root {
                root[0] = new_entry;
            } else {
                if root.contains(&new_entry) {
                    println!("Path already exists. Omitting.");
                } else {
                    root.push(new_entry);
                }
            }
        }
    }
}
