
### Binary Heap Example.

Liz loves cookies, but all cookies must have a sweetness of **at least `k`**. Each operation involves combining the **two least sweet cookies**:

```
new_cookie = least + 2 * second_least
```

This new cookie is added back to the collection. Your task is to determine the **minimum number of such operations** to make all cookies' sweetness ≥ `k`. If it's **impossible**, return `-1`.

---

### Solution Strategy (Min-Heap):

1. Insert all cookie sweetness values into a **min-heap**.
2. While the smallest cookie has sweetness less than `k`:
   - If fewer than 2 cookies are left, return `-1` (can't combine anymore).
   - Pop two smallest cookies and combine them.
   - Push the new cookie back into the heap.
   - Increment operation counter.
3. When all cookies meet the condition, return the operation count.

---

### Rust Code Implementation:

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn cookies(k: i32, A: Vec<i32>) -> i32 {
    let mut heap = BinaryHeap::new();

    // Min-heap using Reverse
    for &sweetness in &A {
        heap.push(Reverse(sweetness));
    }

    let mut operations = 0;

    while let Some(Reverse(min1)) = heap.pop() {
        if min1 >= k {
            return operations;
        }

        if let Some(Reverse(min2)) = heap.pop() {
            let new_sweet = min1 + 2 * min2;
            heap.push(Reverse(new_sweet));
            operations += 1;
        } else {
            // Only one cookie left and it's not sweet enough
            return -1;
        }
    }

    -1
}

fn main() {
    let k = 7;
    let cookies = vec![1, 2, 3, 9, 10, 12];
    let result = cookies(k, cookies);
    println!("{}", result); // Output: 2
}
```

---

### Test Explanation:

**Input:**
```
k = 7  
cookies = [1, 2, 3, 9, 10, 12]
```

**Operations:**
1. Combine 1 + 2*2 → 5 → heap becomes [3, 5, 9, 10, 12]
2. Combine 3 + 2*5 → 13 → heap becomes [9, 10, 12, 13]

All cookies are now ≥ 7 → ✅ **2 operations needed**
