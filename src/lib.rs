#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_tree_has_no_root() {
        let tree = Tree::default();
        assert_eq!(tree.root.is_none(), true);
    }

    #[test]
    fn tree_can_add_root() {
        let mut tree = Tree::default();

        tree.insert(3);
        tree.insert(1);
        tree.insert(2);
        tree.insert(4);
        let root = tree.root.as_ref().unwrap();
        assert_eq!(root.key, 3);

        let one_leaf = root.left.as_ref().unwrap();
        assert_eq!(one_leaf.key, 1);

        let two_leaf = one_leaf.right.as_ref().unwrap();
        assert_eq!(two_leaf.key, 2);

        let four_leaf = root.right.as_ref().unwrap();
        assert_eq!(four_leaf.key, 4);
    }
}

#[derive(Debug, Default)]
pub struct Node {
    key: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn insert(&mut self, key: i32) {
        if key < self.key {
            if self.left.is_none() {
                self.left = Some(Box::new(Node {
                    key,
                    left: None,
                    right: None,
                }));
            } else {
                self.left.as_mut().unwrap().insert(key);
            }
        } else {
            if self.right.is_none() {
                self.right = Some(Box::new(Node {
                    key,
                    left: None,
                    right: None,
                }));
            } else {
                self.right.as_mut().unwrap().insert(key);
            }
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

#[derive(Debug, Default)]
pub struct Tree {
    root: Option<Node>
}

pub trait Bst {
    fn invariant(&self) -> bool {
        true
    }
}

impl Tree {
    pub fn insert(&mut self, key: i32) {
        match self.root {
            None => {
                self.root = Some(Node {
                    key,
                    ..Default::default()
                })
            }
            Some(ref mut node) => {
                if key == node.key {
                    return;
                }
                if key < node.key {
                    match node.left {
                        None => {
                            node.left = Some(Box::new(Node {
                                key,
                                ..Default::default()
                            }))
                        }
                        Some(ref mut left) => {
                            left.insert(key);
                        }
                    }
                } else {
                    match node.right {
                        None => {
                            node.right = Some(Box::new(Node {
                                key,
                                ..Default::default()
                            }))
                        }
                        Some(ref mut right) => {
                            right.insert(key);
                        }
                    }
                }
            }
        }
    }
}
