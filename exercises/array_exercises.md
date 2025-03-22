## Exercise 1

Given an integer array `nums` and an integer `k`, return the `k` most frequent elements. You may return the answer in any order.

### Example 1:
```
Input: nums = [1,1,1,2,2,3], k = 2  
Output: [1,2]
```

### Example 2:
```
Input: nums = [1], k = 1  
Output: [1]
```

### Solution in Rust:
```rust
use std::collections::HashMap;

fn top_k_frequent(nums: Vec<i32>, k: usize) -> Vec<i32> {
    let mut freq_map = HashMap::new();

    // Count frequencies
    for num in nums {
        *freq_map.entry(num).or_insert(0) += 1;
    }

    // Find the max frequency
    let max_freq = freq_map.values().copied().max().unwrap_or(0);

    // Bucket sort: index = frequency, value = list of numbers
    let mut buckets: Vec<Vec<i32>> = vec![vec![]; max_freq as usize + 1];
    for (num, freq) in freq_map {
        buckets[freq as usize].push(num);
    }

    // Collect top K frequent elements
    let mut output = Vec::new();
    let mut count = 0;
    let mut index = max_freq as isize;

    while count < k && index >= 0 {
        if let Some(bucket) = buckets.get(index as usize) {
            output.extend(bucket);
            count += bucket.len();
        }
        index -= 1;
    }

    output.into_iter().take(k).collect()
}
```
