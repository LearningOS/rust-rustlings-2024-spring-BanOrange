/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
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
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        if let Some(ref mut root) = self.root {
            // If the root exists, call insert_recursive on the root node
            Self::insert_recursive(&mut self.root, value);
        } else {
            // If the tree is empty, set the root to the new value
            self.root = Some(Box::new(TreeNode::new(value)));
        }
    }

      // Static method to insert a node into the tree
      fn insert_recursive(node: &mut Option<Box<TreeNode<T>>>, value: T)
      where
          T: Ord,
      {
          match node {
              Some(ref mut n) => {
                  match value.cmp(&n.value) {
                      Ordering::Less => {
                          // Recursively insert into the left subtree
                          Self::insert_recursive(&mut n.left, value);
                      },
                      Ordering::Greater => {
                          // Recursively insert into the right subtree
                          Self::insert_recursive(&mut n.right, value);
                      },
                      Ordering::Equal => {
                          
                      },
                  }
              },
              None => {
                  // If the position is empty, insert the new node here
                  *node = Some(Box::new(TreeNode::new(value)));
              },
          }
      }

    // Search for a value in the BST
    fn search(&mut self, value: T) -> bool {
        //TODO
        if let Some(ref mut n) = self.root{
            return Self::find(&mut self.root,value);
        }else{
            return false;
        }
    }

    fn find(node: &mut Option<Box<TreeNode<T>>>, value: T) -> bool
    where T:Ord,
    {
        match node{
            Some(ref mut n) => {
                match value.cmp(&n.value) {
                    Ordering::Less => {
                        // Recursively insert into the left subtree
                        return Self::find(&mut n.left, value);
                    },
                    Ordering::Greater => {
                        // Recursively insert into the right subtree
                        return Self::find(&mut n.right, value);
                    },
                    Ordering::Equal => {
                        return true;
                    },
                }
            },
            None => {
                return false;
            }
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        match value.cmp(&self.value){
            Ordering::Less => {
                if let Some(left) = &mut self.left{
                    left.insert(value)
                }else{
                    self.left = Some(Box::new(TreeNode::new(value)))
                }
            },
            Ordering::Greater => {
                if let Some(right) = &mut self.right{
                    right.insert(value)
                }else{
                    self.right = Some(Box::new(TreeNode::new(value)))
                }
            },
            Ordering::Equal => {

            }
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


