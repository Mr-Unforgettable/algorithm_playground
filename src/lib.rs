/// Declare the algorithms module, which contains various searching algorithms.
pub mod algorithms {
    /// Import individual modules from the algorithms directory.
    pub mod searching;
    pub mod sorting;
}

/// Re-export the public functions from the searching module.
pub use crate::algorithms::searching::searching::{linear_search, binary_search, Graph, TreeNode};
pub use crate::algorithms::sorting::sorting::{merge_sort, heap_sort, quick_sort, insertion_sort, selection_sort, bubble_sort};

/// Module containing unit tests for the searching algorithms.
#[cfg(test)]
mod search_tests {
    use super::*;
    use super::Graph;
    use super::TreeNode;

    /// Unit tests for the linear_search function.
    #[test]
    fn test_linear_search() {
        // Test with integers
        let arr_int = vec![1, 2, 3, 4, 5];
        assert_eq!(linear_search(&arr_int, &3), Some(2));
    
        // Test with strings
        let arr_str = vec!["apple", "banana", "cherry", "date"];
        assert_eq!(linear_search(&arr_str, &"cherry"), Some(2));

        // Test with characters
        let arr_char = vec!['a', 'b', 'c', 'd', 'e'];
        assert_eq!(linear_search(&arr_char, &'c'), Some(2));

        // Test with custom struct
        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }
        
        let arr_custom = vec![
            Point { x: 1, y: 1 },
            Point { x: 2, y: 2 },
            Point { x: 3, y: 3 },
        ];
        assert_eq!(linear_search(&arr_custom, &Point { x: 3, y: 3 }), Some(2));
    }
    
    /// Unit tests for the binary_search function.
    #[test]
    fn test_binary_search() {
        // Test with integers
        let arr_int = vec![1, 2, 3, 4, 5];
        assert_eq!(binary_search(&arr_int, &3), Some(2));
    
        // Test with strings
        let arr_str = vec!["apple", "banana", "cherry", "date"];
        assert_eq!(binary_search(&arr_str, &"cherry"), Some(2));

        // Test with characters
        let arr_char = vec!['a', 'b', 'c', 'd', 'e'];
        assert_eq!(binary_search(&arr_char, &'c'), Some(2)); 

        // Test with custom struct
        #[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
        struct Point {
            x: i32,
            y: i32,
        }
        
        let arr_custom = vec![
            Point { x: 1, y: 1 },
            Point { x: 2, y: 2 },
            Point { x: 3, y: 3 },
        ];
        assert_eq!(binary_search(&arr_custom, &Point { x: 3, y: 3 }), Some(2));
    } 

    #[test]
    fn test_graph_dfs_and_bfs() {
        // Test Graph with usize
        let mut graph_usize = Graph::new(5);
        graph_usize.add_edge(0, 1);
        graph_usize.add_edge(0, 2);
        graph_usize.add_edge(1, 3);
        graph_usize.add_edge(2, 4);
        assert!(graph_usize.dfs(0, 4));
        assert!(graph_usize.bfs(0, 4));

        // Test Graph with char
        let mut graph_char = Graph::new(5);
        graph_char.add_edge(0, 1);
        graph_char.add_edge(0, 2);
        graph_char.add_edge(1, 3);
        graph_char.add_edge(2, 4);
        assert!(graph_char.dfs(0, 4));
        assert!(graph_char.bfs(0, 4));
    }

    #[test]
    fn test_tree_contains() {
        // Test TreeNode with usize
        let mut root_usize = TreeNode::new(5);
        let left_child_usize = TreeNode::new(3);
        let right_child_usize = TreeNode::new(7);
        root_usize.left = Some(Box::new(left_child_usize));
        root_usize.right = Some(Box::new(right_child_usize));
        assert!(root_usize.contains(&3));
        assert!(!root_usize.contains(&4));

        // Test TreeNode with char
        let mut root_char = TreeNode::new('c');
        let left_child_char = TreeNode::new('b');
        let right_child_char = TreeNode::new('d');
        root_char.left = Some(Box::new(left_child_char));
        root_char.right = Some(Box::new(right_child_char));
        assert!(root_char.contains(&'b'));
        assert!(!root_char.contains(&'a'));
    }
}

#[cfg(test)]
mod sort_tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut arr = [3, 2, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);

        let mut arr = ['c', 'b', 'a'];
        merge_sort(&mut arr);
        assert_eq!(arr, ['a', 'b', 'c']);
    }

    #[test]
    fn test_heap_sort() {
        let mut arr = [3, 2, 1];
        heap_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);

        let mut arr = ['c', 'b', 'a'];
        heap_sort(&mut arr);
        assert_eq!(arr, ['a', 'b', 'c']); 
    }

    #[test]
    fn test_quick_sort() {
        let mut arr = [3, 2, 1];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);

        let mut arr = ['c', 'b', 'a'];
        quick_sort(&mut arr);
        assert_eq!(arr, ['a', 'b', 'c']);
    }

    #[test]
    fn test_insertion_sort() {
        let mut arr = [3, 2, 1];
        insertion_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);

        let mut arr = ['c', 'b', 'a'];
        insertion_sort(&mut arr);
        assert_eq!(arr, ['a', 'b', 'c']);
    }

    #[test]
    fn test_selection_sort() {
        let mut arr = [3, 2, 1];
        selection_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);

        let mut arr = ['c', 'b', 'a'];
        selection_sort(&mut arr);
        assert_eq!(arr, ['a', 'b', 'c']);
    }

    #[test]
    fn test_bubble_sort() {
        let mut arr = [3, 2, 1];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);

        let mut arr = ['c', 'b', 'a'];
        bubble_sort(&mut arr);
        assert_eq!(arr, ['a', 'b', 'c']);
    }
}