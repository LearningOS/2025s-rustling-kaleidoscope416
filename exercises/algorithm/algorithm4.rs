use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord+Clone,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord+Clone,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord+Clone,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord+Clone,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        //DONE
       if let None = self.root{
        let mut t_n = TreeNode::new(value);
        self.root = Some(Box::new(t_n));
       }
       else{
        self.root.as_mut().unwrap().insert(value);
       }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        fn search_node<T: Ord+Clone>(node: &Option<Box<TreeNode<T>>>, value: &T) -> bool {
            match node {
                None => false,
                Some(node) => match value.cmp(&node.value) {
                    Ordering::Equal => true,
                    Ordering::Less => search_node(&node.left, value),
                    Ordering::Greater => search_node(&node.right, value),
                },
            }
        }

        search_node(&self.root, &value)
    }
}


impl<T> TreeNode<T>
where
    T: Ord+Clone,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        //DONE
        let mut new_tn = TreeNode::new(value.clone());
        if new_tn.value<self.value{
            if self.left.is_none(){
                self.left = Some(Box::new(new_tn));
            }
            else {
                self.left.as_mut().unwrap().insert(value);
            }
        }
        else if new_tn.value>self.value{
            if self.right.is_none(){
                self.right = Some(Box::new(new_tn));
            }
            else {
                self.right.as_mut().unwrap().insert(value);
            }
        }
        else{
            return
        }

       
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


