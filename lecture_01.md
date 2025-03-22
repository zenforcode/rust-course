
# Getting Started with Rust: Ownership and Memory Management

In this course, we aim to get you proficient with Rust as quickly as possible. We will dive into what distinguishes Rust from other programming languages, starting with Rust’s Ownership Model. You’ll learn the basics of Rust programming by proxy. Let's get started!

---

## Manual vs. Automated Memory Management

Before we delve into Rust's unique features, it's important to understand the context of memory management in programming languages. There are generally two types:

### Manual Memory Management Languages

**Example: C**

```c
#include <stdlib.h>

int main() {
    int *array = (int*)malloc(10 * sizeof(int));
    free(array);
    return 0;
}
```

In manual memory management languages like C, you are responsible for allocating and freeing memory. For instance, you use `malloc` to allocate memory and `free` to release it. This requires careful management to prevent memory leaks and other issues.

---

### Automated Memory Management Languages

**Examples: Java, Kotlin, PHP, JavaScript, etc.**

```java
public class Main {
    public static void main(String[] args) {
        int[] array = new int[10];
    }
}
```

In automated memory management languages, the memory is managed for you through a garbage collector. This means you can create objects and rely on the language runtime to free up memory when it's no longer needed, reducing the risk of memory leaks.

---

## How About in Rust?

So, where does Rust fit in? Do we have to manage memory manually, or is there a garbage collector?

Let's look at the same example in Rust:

```rust
fn main() {
    let array = vec![0; 10];
}
```

At first glance, it might seem like Rust behaves like a Java-like language since there's no explicit `free` statement. However, Rust doesn't fit neatly into either the manual or automated memory management categories.

- Rust **doesn't have a garbage collector**.
- Rust **doesn't require manual memory management** like C.

Instead, Rust introduces a new category:

### Rust-like Languages

Rust achieves memory safety and performance without a garbage collector through its **Ownership Model**.

---

## Rust’s Ownership Model

Rust's Ownership Model is based on three core principles:

1. Every value has an owner.
2. There can only be one owner at a time for a value.
3. Values get dropped when their owner goes out of scope.

---

### Every Value Has Only One Owner

This rule can lead to some initially unintuitive results. Consider the following code:

```rust
fn main() {
    let some_string = String::from("Hello, World!");
    let _some_other_string = some_string;
    println!("{}", some_string);
}
```

What do you think will happen when we run this code?

If you expect it to print `"Hello, World!"`, think again. This code will **not compile** and will produce the following error:

```
error[E0382]: borrow of moved value: `some_string`
 --> src/main.rs:4:20
  |
2 |     let some_string = String::from("Hello, World!");
  |         ----------- move occurs because `some_string` has type `String`, which does not implement the `Copy` trait
3 |     let _some_other_string = some_string;
  |                              ----------- value moved here
4 |     println!("{}", some_string);
  |                    ^^^^^^^^^^^ value borrowed here after move
```

The error indicates that `some_string` has been moved to `_some_other_string`, and thus `some_string` is no longer valid.

---

## Variables and Ownership

In Rust:

- Each variable is an **owner** of its data.
- Assigning one variable to another **moves** the data, transferring ownership.
- After a move, the original variable can no longer be used.

This might seem inconvenient, but it's crucial for Rust's safety guarantees. It allows Rust to offer **performance similar to C** while ensuring **memory safety** like Java.

---

## Understanding the Copy Trait

Looking back at the error message, you might notice it mentions the **Copy trait**. What does this mean?

Some types in Rust, like integers, are **Copy** types. They can be duplicated simply by copying their bits, which is a cheap operation.

**Example:**

```rust
fn main() {
    let some_int = 42;
    let _some_other_int = some_int;
    println!("{}", some_int);
}
```

This code compiles and runs as expected, printing `42`. That's because integers implement the `Copy` trait, so they are **copied**, not moved.

---

## Ways Variables Interact

There are three primary ways variables can interact in Rust:

### 1. Move

- Applies to types that occupy a variable amount of memory, like `String`.
- Ownership is **transferred** to the new variable.
- Original variable is **invalidated**.

### 2. Copy

- Applies to types with a fixed size known at compile time, like `i32`.
- Data is **copied** to the new variable.
- **Both** variables can be used independently.

### 3. Clone

- Explicitly creates a **deep copy** of the data.
- Can be used with types that do not implement `Copy`.
- Original and cloned variables are **independent**.

---

## Using `clone`

To fix our earlier `String` example, we can use the `clone()` method:

```rust
fn main() {
    let some_string = String::from("Hello, World!");
    let _some_other_string = some_string.clone();
    println!("{}", some_string);
}
```

By calling `clone()`, we explicitly create a copy of the data on the heap. Now, both `some_string` and `_some_other_string` own their data, and the code compiles successfully, printing `"Hello, World!"`.

---

## Conclusion

We've covered **Rust's Ownership Model** and how it differs from other languages. Understanding ownership is fundamental to writing efficient and safe Rust code.

---

### Next Up

Join me next time, where we will explore:

- Rust's **borrow checker**
- **Mutability**
- **Scopes**

Thank you for being part of this journey into Rust!
