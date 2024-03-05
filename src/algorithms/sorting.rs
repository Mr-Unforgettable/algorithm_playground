//! # Sorting
//! 
//! `sorting` is a library for common sorting algorithms implemented in rust.

pub mod sorting {
    /// Sorts a slice using the merge sort algorithm.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use algorithm_playground::algorithms::sorting::sorting::merge_sort;
    /// 
    /// let mut arr = [3, 2, 1];
    /// merge_sort(&mut arr);
    /// assert_eq!(arr, [1, 2, 3]);
    /// ```
    
    pub fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
        let len = arr.len();
        
        // Base case: If the slice has 0 or 1 element, it's already sorted
        if len <= 1 {
            return;
        }
        
        let mid = len / 2;
        let mut left = arr[..mid].to_vec(); // Create a copy of the left half
        let mut right = arr[mid..].to_vec(); // Create a copy of the right half
    
        merge_sort(&mut left); // Recursively sort the left half
        merge_sort(&mut right); // Recursively sort the right half
    
        // Merge the sorted halves back into the original slice
        let mut i = 0;
        let mut j = 0;
        let mut k = 0;
        while i < left.len() && j < right.len() {
            if left[i] <= right[j] {
                arr[k] = left[i].clone();
                i += 1;
            } else {
                arr[k] = right[j].clone();
                j += 1;
            }
            k += 1;
        }
    
        // Copy any remaining elements from the left half
        while i < left.len() {
            arr[k] = left[i].clone();
            i += 1;
            k += 1;
        }
    
        // Copy any remaining elements from the right half
        while j < right.len() {
            arr[k] = right[j].clone();
            j += 1;
            k += 1;
        }
    }

    /// Sort a slice using heap sort algorithm.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use algorithm_playground::algorithms::sorting::sorting::heap_sort;
    /// 
    /// let mut arr = [3, 2, 1];
    /// heap_sort(&mut arr);
    /// assert_eq!(arr, [1, 2, 3]);
    /// ```
    
    pub fn heap_sort<T: Ord>(arr: &mut[T]) {
        if arr.len() <= 1 {
            return;
        }

        // Build heap
        let len = arr.len();
        for i in (0..len / 2).rev() {
            heapify(arr, len, i);
        }

        // Extract elements from heap
        for i in (1..len).rev() {
            arr.swap(0, i);
            heapify(arr, i, 0);
        }
    }

    fn heapify<T: Ord>(arr: &mut [T], n: usize, i: usize) {
        let mut largest = i;
        let left = 2 * i + 1;
        let right = 2 * i + 2;

        if left < n && arr[left] > arr[largest] {
            largest = left;
        }

        if right < n && arr[right] > arr[largest] {
            largest = right;
        }

        if largest != i {
            arr.swap(i, largest);
            heapify(arr, n, largest);
        }
    }

    /// Sorts a slice using the quick sort algorithm.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use algorithm_playground::algorithms::sorting::sorting::quick_sort;
    /// 
    /// let mut arr = [3, 2, 1];
    /// quick_sort(&mut arr);
    /// assert_eq!(arr, [1, 2, 3]);
    /// ```
    
    pub fn quick_sort<T: Ord>(arr: &mut [T]) {
        if arr.len() <= 1 {
            return;
        }

        let pivot_index = partition(arr);
        quick_sort(&mut arr[0..pivot_index]);
        quick_sort(&mut arr[pivot_index + 1..]);
    }

    fn partition<T: Ord>(arr: &mut [T]) -> usize {
        let len = arr.len();
        let pivot_index = len / 2;
        arr.swap(pivot_index, len - 1);

        let mut i = 0;
        for j in 0..len - 1 {
            if arr[j] <= arr[len - 1] {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, len - 1);
        i
    }

    /// Sorts a slice using the insertion sort algorithm.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use algorithm_playground::algorithms::sorting::sorting::insertion_sort;
    /// 
    /// let mut arr = [3, 2, 1];
    /// insertion_sort(&mut arr);
    /// assert_eq!(arr, [1, 2, 3]);
    /// ```
    
    pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
        let len = arr.len();
        for i in 1..len {
            let mut j = i;
            while j > 0 && arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
                j -= 1;
            }
        }
    }

    /// Sorts a slice using the selection sort algorithm.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use algorithm_playground::algorithms::sorting::sorting::selection_sort;
    /// 
    /// let mut arr = [3, 2, 1];
    /// selection_sort(&mut arr);
    /// assert_eq!(arr, [1, 2, 3]);
    /// ```
    
    pub fn selection_sort<T: Ord>(arr: &mut [T]) {
        let len = arr.len();
        for i in 0..len {
            let mut min_index = i;
            for j in i + 1..len {
                if arr[j] < arr[min_index] {
                    min_index = j;
                }
            }
            if min_index != i {
                arr.swap(i, min_index);
            }
        }
    }

    /// Sorts a slice using the bubble sort algorithm.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use algorithm_playground::algorithms::sorting::sorting::bubble_sort;
    /// 
    /// let mut arr = [3, 2, 1];
    /// bubble_sort(&mut arr);
    /// assert_eq!(arr, [1, 2, 3]);
    /// ```
    
    pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
        let len = arr.len();
        for i in 0..len {
            for j in 0..len - 1 - i {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                }
            }
        }
    }
}