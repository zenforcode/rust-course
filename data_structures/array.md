# Fixed size arrty
A fixed-size array, denoted [T; N], for the element type, T, and the non-negative compile-time constant size, N.
There are two syntactic forms for creating an array:

- A list with each element, i.e., [x, y, z].
- A repeat expression [expr; N] where N is how many times to repeat expr in the array. expr must either be:
    - A value of a type implementing the Copy trait
    - A const value

```rust
use std::array;
use std::cmp::Ordering;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

const fn sum_array(arr: [i32; 5]) -> i32 {
    let mut sum = 0;
    let mut i = 0;
    while i < 5 {
        sum += arr[i];
        i += 1;
    }
    sum
}

fn main() {
    let mut numbers = [5, 3, 8, 1, 4];
    let points = [Point { x: 1, y: 2 }, Point { x: 3, y: 4 }, Point { x: 5, y: 6 }];

    // Structured bindings
    for Point { x, y } in &points {
        println!("({}, {})", x, y);
    }

    const CONST_NUMBERS: [i32; 5] = [1, 2, 3, 4, 5];
    const TOTAL_SUM: i32 = sum_array(CONST_NUMBERS);
    println!("\nCompile-time sum of array elements: {}", TOTAL_SUM);

    numbers.sort();
    println!("\nSorted numbers: {:?}", numbers);

    let search_for = 3;
    if numbers.contains(&search_for) {
        println!("\nFound {} in the array.", search_for);
    } else {
        println!("\nDidn't find {} in the array.", search_for);
    }
}
```