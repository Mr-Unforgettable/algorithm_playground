/// This module contains implementations of searching algorithms.
pub mod searching {
    use std::collections::{HashSet, VecDeque};

    /// Performs a linear search on a slice to find a target element.
    ///
    /// This function iterates over the elements of the slice in order and compares each element
    /// to the target element using the `PartialEq` trait. If a match is found, the index of the
    /// matching element is returned.
    ///
    /// # Arguments
    ///
    /// * `arr` - A slice of elements to search through.
    /// * `target` - The target element to search for.
    ///
    /// # Returns
    ///
    /// An `Option` containing the index of the target element if found, or `None` if the target
    /// element is not present in the slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use algorithm_playground::algorithms::searching::searching::linear_search;
    ///
    /// let arr = vec![1, 2, 3, 4, 5];
    /// assert_eq!(linear_search(&arr, &3), Some(2));
    /// ```
    
    pub fn linear_search<T: PartialEq>(arr: &[T], target: &T) -> Option<usize> {
        for (index, item) in arr.iter().enumerate() {
            if item == target {
                return Some(index)
            }
        }
        None
    }

    /// Performs a binary search on a sorted slice to find a target element.
    ///
    /// This function assumes that the slice is sorted in ascending order and uses the `Ord` trait
    /// to compare elements. It repeatedly divides the search space in half until the target element
    /// is found or the search space is exhausted.
    ///
    /// # Arguments
    ///
    /// * `arr` - A sorted slice of elements to search through.
    /// * `target` - The target element to search for.
    ///
    /// # Returns
    ///
    /// An `Option` containing the index of the target element if found, or `None` if the target
    /// element is not present in the slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use algorithm_playground::algorithms::searching::searching::binary_search;
    ///
    /// let arr = vec![1, 2, 3, 4, 5];
    /// assert_eq!(binary_search(&arr, &3), Some(2));
    /// ```
    
    pub fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
        let mut low = 0;
        let mut high = arr.len() - 1;

        while low <= high {
            let mid = (low + high) / 2;
            if arr[mid] == *target {
                return Some(mid);
            }
            if arr[mid] < *target {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        None
    }

    /// Example graph representation using an adjacency list
    pub struct Graph {
       pub edges: Vec<Vec<usize>>,
    }
    
    impl Graph {
        /// Creates a new graph with the specified number of nodes.
        ///
        /// # Arguments
        ///
        /// * `num_nodes` - The number of nodes in the graph.
        ///
        /// # Examples
        ///
        /// ```
        /// use algorithm_playground::algorithms::searching::searching::Graph;
        /// 
        /// let mut graph = Graph::new(5);
        /// graph.add_edge(0, 1);
        /// graph.add_edge(0, 2);
        /// graph.add_edge(1, 3);
        /// graph.add_edge(2, 4);
        /// ```
        pub fn new(num_nodes: usize) -> Self {
            Graph {
                edges: vec![vec![]; num_nodes],
            }
        }
    
        /// Adds an undirected edge between two nodes.
        ///
        /// # Arguments
        ///
        /// * `from` - The index of the starting node.
        /// * `to` - The index of the ending node.
        ///
        /// # Examples
        ///
        /// ```
        /// use algorithm_playground::algorithms::searching::searching::Graph;
        /// 
        /// let mut graph = Graph::new(5);
        /// graph.add_edge(0, 1);
        /// graph.add_edge(0, 2);
        /// graph.add_edge(1, 3);
        /// graph.add_edge(2, 4);
        /// ```
        pub fn add_edge(&mut self, from: usize, to: usize) {
            self.edges[from].push(to);
            self.edges[to].push(from); // Assuming undirected graph
        }
    
        /// Performs a Depth-First Search (DFS) from the start node to find the target node.
        ///
        /// Returns true if the target node is found, otherwise false.
        ///
        /// # Arguments
        ///
        /// * `start` - The index of the starting node.
        /// * `target` - The index of the target node to search for.
        ///
        /// # Complexity
        ///
        /// The time complexity of DFS is O(V + E), where V is the number of vertices
        /// and E is the number of edges in the graph.
        ///
        /// # Examples
        ///
        /// ```
        /// use algorithm_playground::algorithms::searching::searching::Graph;
        /// 
        /// let mut graph = Graph::new(5);
        /// graph.add_edge(0, 1);
        /// graph.add_edge(0, 2);
        /// graph.add_edge(1, 3);
        /// graph.add_edge(2, 4);
        /// assert!(graph.dfs(0, 4));
        /// ```
        pub fn dfs(&self, start: usize, target: usize) -> bool {
            let mut visited = HashSet::new();
            self.dfs_recursive(start, target, &mut visited)
        }
    
        pub fn dfs_recursive(&self, current: usize, target: usize, visited: &mut HashSet<usize>) -> bool {
            if current == target {
                return true;
            }
            if visited.contains(&current) {
                return false;
            }
            visited.insert(current);
            for &neighbor in &self.edges[current] {
                if self.dfs_recursive(neighbor, target, visited) {
                    return true;
                }
            }
            false
        }
    
        /// Performs a Breadth-First Search (BFS) from the start node to find the target node.
        ///
        /// Returns true if the target node is found, otherwise false.
        ///
        /// # Arguments
        ///
        /// * `start` - The index of the starting node.
        /// * `target` - The index of the target node to search for.
        ///
        /// # Complexity
        ///
        /// The time complexity of BFS is O(V + E), where V is the number of vertices
        /// and E is the number of edges in the graph.
        ///
        /// # Examples
        ///
        /// ```
        /// use algorithm_playground::algorithms::searching::searching::Graph;
        /// 
        /// let mut graph = Graph::new(5);
        /// graph.add_edge(0, 1);
        /// graph.add_edge(0, 2);
        /// graph.add_edge(1, 3);
        /// graph.add_edge(2, 4);
        /// assert!(graph.bfs(0, 4));
        /// ```
        pub fn bfs(&self, start: usize, target: usize) -> bool {
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back(start);
            while let Some(current) = queue.pop_front() {
                if current == target {
                    return true;
                }
                if visited.contains(&current) {
                    continue;
                }
                visited.insert(current);
                for &neighbor in &self.edges[current] {
                    queue.push_back(neighbor);
                }
            }
            false
        }
    }
    
    /// Assuming a simple binary search tree structure
    pub struct TreeNode<T> {
       pub value: T,
       pub left: Option<Box<TreeNode<T>>>,
       pub right: Option<Box<TreeNode<T>>>,
    }
    
    impl<T: Ord> TreeNode<T> {
        /// Creates a new tree node with the specified value.
        ///
        /// # Arguments
        ///
        /// * `value` - The value of the new tree node.
        ///
        /// # Examples
        ///
        /// ```
        /// use algorithm_playground::algorithms::searching::searching::TreeNode;
        /// 
        /// let node = TreeNode::new(5);
        /// ```
        pub fn new(value: T) -> Self {
            TreeNode {
                value,
                left: None,
                right: None,
            }
        }
    
        /// Checks if the tree contains a node with the specified value.
        ///
        /// Returns true if the value is found, otherwise false.
        ///
        /// # Arguments
        ///
        /// * `target` - The value to search for in the tree.
        ///
        /// # Complexity
        ///
        /// The time complexity of contains method is O(log n) for balanced binary search trees,
        /// where n is the number of nodes in the tree. However, it can degrade to O(n) for
        /// unbalanced trees.
        ///
        /// # Examples
        ///
        /// ```
        /// use algorithm_playground::algorithms::searching::searching::TreeNode;
        /// 
        /// let mut root = TreeNode::new(5);
        /// let left_child = TreeNode::new(3);
        /// let right_child = TreeNode::new(7);
        /// root.left = Some(Box::new(left_child));
        /// root.right = Some(Box::new(right_child));
        ///
        /// assert!(root.contains(&3));
        /// assert!(!root.contains(&4));
        /// ```
        pub fn contains(&self, target: &T) -> bool {
            if *target == self.value {
                return true;
            }
            if *target < self.value {
                if let Some(ref left) = self.left {
                    return left.contains(target);
                }
            } else {
                if let Some(ref right) = self.right {
                    return right.contains(target);
                }
            }
            false
        }
    }
}
