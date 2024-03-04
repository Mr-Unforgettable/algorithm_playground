/// Declare the algorithms module, which contains various searching algorithms.
pub mod algorithms {
    /// Import individual modules from the algorithms directory.
    pub mod searching;
}

/// Re-export the public functions from the searching module.
pub use crate::algorithms::searching::searching::{linear_search, binary_search};

/// Module containing unit tests for the searching algorithms.
#[cfg(test)]
mod tests {
    use super::*;

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
}
