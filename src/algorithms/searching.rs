/// This module contains implementations of searching algorithms.
pub mod searching {
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
            } else if arr[mid] < *target {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        None
    }
}
