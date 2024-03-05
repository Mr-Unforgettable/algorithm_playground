# Algorithm Playground

Algorithm Playground is a Rust library that provides implementations of various searching algorithms.

## Features

- Includes implementations of search, sort algorithms.
- Works with any type that implements the `PartialEq` or `Ord` trait.

## Installation

To use Algorithm Playground in your Rust project, add it as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
algorithm_playground = "1.0.1"
```

## Usage
Here's an example of how to use the searching algorithms in your Rust code:

```
use algorithm_playground::algorithms::searching::searching::{linear_search, binary_search};

fn main() {
    let arr = vec![1, 2, 3, 4, 5];
    
    // Perform linear search
    match linear_search(&arr, &3) {
        Some(index) => println!("Found 3 at index {}", index),
        None => println!("3 not found"),
    }

    // Perform binary search (requires sorted array)
    match binary_search(&arr, &3) {
        Some(index) => println!("Found 3 at index {}", index),
        None => println!("3 not found"),
    }
}
```

## Contributing
Contributions to Algorithm Playground are welcome! If you find a bug or have a feature request, please open an issue on GitHub. Pull requests are also appreciated.

## License
This project is licensed under the MIT License - see the LICENSE file for details.
