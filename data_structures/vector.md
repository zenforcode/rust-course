# Vector in Rust
In Rust, ``Vec<T>`` is a growable, heap-allocated vector. It's one of the most commonly used collections and is analogous to std::vector in C++ or lists in other languages. A Vec allows you to store a sequence of values that can dynamically grow or shrink at runtime.

## Example 1
```rust
let mut numbers: Vec<i32> = Vec::new();
numbers.push(1);
numbers.push(2);
numbers.push(3);
```
You can also initialize vec using a macro:
```rust
let numbers = vec![1, 2, 3, 4, 5];
```
## Example 2

```rust
fn main() {
    let mut n = 0;
    let numbers: Vec<i32> = (0..10).map(|_| {
        n += 1;
        n
    }).collect();

    for num in &numbers {
        print!("{} ", num);
    }
    println!();
}
```
