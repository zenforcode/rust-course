## Hashmap and HashSet
```rust
use std::collections::{HashMap, HashSet};

fn find_pairs_with_sum(nums: Vec<i32>, target: i32) -> Vec<(i32, i32)> {
    let mut seen = HashSet::new();      // To store elements we've seen
    let mut output = HashSet::new();    // To store unique pairs

    for &num in &nums {
        let complement = target - num;
        if seen.contains(&complement) {
            let pair = if num < complement {
                (num, complement)
            } else {
                (complement, num)
            };
            output.insert(pair);
        }
        seen.insert(num);
    }

    output.into_iter().collect()
}

fn main() {
    let numbers = vec![2, 4, 3, 5, 7, 8, -1, 1, 6];
    let target = 7;
    let pairs = find_pairs_with_sum(numbers, target);

    println!("Pairs that sum to {}:", target);
    for (a, b) in pairs {
        println!("({}, {})", a, b);
    }
}
```
