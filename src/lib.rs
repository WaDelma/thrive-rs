use std::mem;
use std::fmt;

// TODO: Add benchmarks!

pub struct Trie<V> {
    root: Node<V>,
    size: usize,
}

impl<V> fmt::Debug for Trie<V> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Trie")
            .field("size", &self.size)
            .field("root", &self.root)
            .finish()
    }
}

impl<V> Trie<V> {
    pub fn new() -> Trie<V> {
        Self {
            root: Node::empty16(),
            size: 0,
        }
    }

    pub fn insert(&mut self, key: usize, value: V) -> Option<V> {
        use Node::*;
        let mut cur = &mut self.root;
        let mut depth = 0;
        loop {
            match cur {
                Node16(children) => {
                    let len = children.len();
                    let idx = index(key, depth, len);
                    if children[idx].is_none() {
                        children[idx] = Some(Leaf(key, value));
                    } else if let Some(node) = &mut children[idx] {
                        cur = node;
                        depth += 1;
                        continue;
                    }
                    self.size += 1;
                    return None;
                },
                Leaf(k, v) => {
                    if *k == key {
                        return Some(mem::replace(v, value));
                    }
                    self.size += 1;
                    let new_node = Node::empty16();
                    let old = mem::replace(cur, new_node);
                    if let Node16(cur) = cur {
                        if let Leaf(k, v) = old {
                            let len = cur.len();
                            cur[index(k, depth, len)] = Some(Leaf(k, v));
                            cur[index(key, depth, len)] = Some(Leaf(key, value));
                            return None;
                        }
                    }
                    return None;
                },
            }
        }
    }

    pub fn get(&self, key: usize) -> Option<&V> {
        use Node::*;
        let mut cur = &self.root;
        let mut depth = 0;
        loop {
            match cur {
                Node16(children) => {
                    let len = children.len();
                    let idx = index(key, depth, len);
                    if let Some(node) = &children[idx] {
                        cur = node;
                        depth += 1;
                    } else {
                        return None;
                    }
                },
                Leaf(k, v) => {
                    if *k == key {
                        return Some(v);
                    }
                    todo!();
                },
            }
        }
    }

    pub fn get_mut(&mut self, key: usize) -> Option<&mut V> {
        None
    }

    pub fn remove(&mut self, key: usize) -> Option<V> {
        None
    }

    pub fn len(&self) -> usize {
        self.size
    }
}

pub fn index(key: usize, depth: usize, len: usize) -> usize {
    (key >> 4 * depth) & (len - 1)
}

enum Node<V> {
    Node16(Box<[Option<Node<V>>]>),
    Leaf(usize, V)
}

impl<V> fmt::Debug for Node<V> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use Node::*;
        match self {
            Node16(children) => {
                fmt.write_str("N16(")?;
                write!(fmt, "{}", 
                    children.iter()
                        .map(|c| if let Some(c) = c {
                            format!("{:?},", c)
                        } else {
                            "o,".to_string()
                        })
                        .collect::<String>()
                )?;
                fmt.write_str(")\n")
            },
            Leaf(k, _) => write!(fmt, "{}", k),
        }
    }
}

impl<V> Node<V> {
    fn empty16() -> Node<V> {
        Node::Node16((0..16).map(|_| None).collect::<Vec<_>>().into_boxed_slice())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn creating_empty_trie_works() {
        let trie = crate::Trie::<String>::new();
        assert_eq!(trie.len(), 0);
    }

    #[test]
    fn inserting_1_entry_works() {
        let mut trie = crate::Trie::new();
        assert_eq!(trie.insert(0, 0), None);
        assert_eq!(trie.len(), 1);
        assert_eq!(trie.get(0), Some(&0));
        assert_eq!(trie.get(1), None);
    }


    #[test]
    fn reinserting_1_entry_works() {
        let mut trie = crate::Trie::new();
        assert_eq!(trie.insert(0, 0), None);
        assert_eq!(trie.insert(0, 1), Some(0));
        assert_eq!(trie.len(), 1);
        assert_eq!(trie.get(0), Some(&1));
    }

    #[test]
    fn inserting_2_entries_works() {
        let mut trie = crate::Trie::new();
        assert_eq!(trie.insert(0, 0), None);
        assert_eq!(trie.insert(1, 1), None);
        assert_eq!(trie.len(), 2);
        assert_eq!(trie.get(0), Some(&0));
        assert_eq!(trie.get(1), Some(&1));
        assert_eq!(trie.get(2), None);
    }

    #[test]
    fn inserting_2_entries_in_reverse_works() {
        let mut trie = crate::Trie::new();
        assert_eq!(trie.insert(1, 1), None);
        assert_eq!(trie.insert(0, 0), None);
        assert_eq!(trie.len(), 2);
        assert_eq!(trie.get(0), Some(&0));
        assert_eq!(trie.get(1), Some(&1));
        assert_eq!(trie.get(2), None);
    }

    #[test]
    fn inserting_3_entries_works() {
        let mut trie = crate::Trie::new();
        assert_eq!(trie.insert(0, 0), None);
        assert_eq!(trie.insert(1, 1), None);
        assert_eq!(trie.insert(2, 2), None);
        assert_eq!(trie.len(), 3);
        assert_eq!(trie.get(0), Some(&0));
        assert_eq!(trie.get(1), Some(&1));
        assert_eq!(trie.get(2), Some(&2));
        assert_eq!(trie.get(3), None);
    }

    #[test]
    fn inserting_3_entries_in_reverse_works() {
        let mut trie = crate::Trie::new();
        assert_eq!(trie.insert(2, 2), None);
        assert_eq!(trie.insert(1, 1), None);
        assert_eq!(trie.insert(0, 0), None);
        assert_eq!(trie.len(), 3);
        assert_eq!(trie.get(0), Some(&0));
        assert_eq!(trie.get(1), Some(&1));
        assert_eq!(trie.get(2), Some(&2));
        assert_eq!(trie.get(3), None);
    }

    #[test]
    fn inserting_32_entries_works() {
        let mut trie = crate::Trie::new();
        for i in 0..32 {
            assert_eq!(trie.insert(i, i), None);
        }
        assert_eq!(trie.len(), 32);
        for i in 0..32 {
            assert_eq!(trie.get(i), Some(&i));
        }
        assert_eq!(trie.get(33), None);
    }

    #[test]
    fn inserting_32_entries_in_reverse_works() {
        let mut trie = crate::Trie::new();
        for i in (0..32).rev() {
            assert_eq!(trie.insert(i, i), None);
        }
        assert_eq!(trie.len(), 32);
        for i in 0..32 {
            assert_eq!(trie.get(i), Some(&i));
        }
        assert_eq!(trie.get(33), None);
    }
}
