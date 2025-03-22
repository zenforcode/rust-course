# 🧠 LRU Cache in Rust Using Doubly Linked List

This is a full implementation of an **LRU (Least Recently Used) Cache** in Rust using:

- `HashMap` for **O(1)** key lookup
- A custom **doubly linked list** for **O(1)** insert/move/delete operations

Rust’s ownership system makes this kind of data structure interesting and challenging to build. We'll use `Rc<RefCell<...>>` to enable shared and mutable access to nodes.

---

## 📦 Crates Used

We only use the **standard library**:
- `std::collections::HashMap`
- `std::rc::Rc`
- `std::cell::RefCell`

---

## 🧩 Problem Definition

```rust
let mut lru = LRUCache::new(2);
lru.put(1, 1);
lru.put(2, 2);
assert_eq!(lru.get(1), 1); // access key 1 -> most recently used
lru.put(3, 3); // evicts key 2
assert_eq!(lru.get(2), -1);
lru.put(4, 4); // evicts key 1
assert_eq!(lru.get(1), -1);
assert_eq!(lru.get(3), 3);
assert_eq!(lru.get(4), 4);
```

---

## 🧱 Node and List Implementation

```rust
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    key: i32,
    val: i32,
    prev: Option<Weak<RefCell<Node>>>,
    next: Link,
}

impl Node {
    fn new(key: i32, val: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            key,
            val,
            prev: None,
            next: None,
        }))
    }
}
```

---

## 🔁 Doubly Linked List with Head/Tail

We use a dummy `head` and `tail` node to simplify logic.

```rust
struct DoublyLinkedList {
    head: Link,
    tail: Link,
}

impl DoublyLinkedList {
    fn new() -> Self {
        let head = Node::new(0, 0);
        let tail = Node::new(0, 0);

        head.borrow_mut().next = Some(tail.clone());
        tail.borrow_mut().prev = Some(Rc::downgrade(&head));

        Self {
            head: Some(head),
            tail: Some(tail),
        }
    }

    fn push_front(&mut self, node: Rc<RefCell<Node>>) {
        let first = self.head.as_ref().unwrap().borrow().next.clone();
        node.borrow_mut().next = first.clone();
        node.borrow_mut().prev = Some(Rc::downgrade(self.head.as_ref().unwrap()));

        if let Some(first_node) = first {
            first_node.borrow_mut().prev = Some(Rc::downgrade(&node));
        }

        self.head.as_ref().unwrap().borrow_mut().next = Some(node);
    }

    fn remove(&mut self, node: &Rc<RefCell<Node>>) {
        let prev = node.borrow().prev.as_ref().and_then(|w| w.upgrade());
        let next = node.borrow().next.clone();

        if let Some(prev_node) = prev {
            prev_node.borrow_mut().next = next.clone();
        }

        if let Some(next_node) = next {
            next_node.borrow_mut().prev = node.borrow().prev.clone();
        }
    }

    fn pop_tail(&mut self) -> Option<Rc<RefCell<Node>>> {
        let last = self.tail.as_ref()?.borrow().prev.as_ref()?.upgrade()?;
        if Rc::ptr_eq(&last, self.head.as_ref().unwrap()) {
            return None;
        }
        self.remove(&last);
        Some(last)
    }
}
```

---

## 🧠 LRU Cache

```rust
struct LRUCache {
    capacity: usize,
    map: HashMap<i32, Rc<RefCell<Node>>>,
    list: DoublyLinkedList,
}

impl LRUCache {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            map: HashMap::new(),
            list: DoublyLinkedList::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.map.get(&key) {
            let val = node.borrow().val;
            self.list.remove(node);
            self.list.push_front(node.clone());
            val
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.map.get(&key) {
            node.borrow_mut().val = value;
            self.list.remove(node);
            self.list.push_front(node.clone());
        } else {
            if self.map.len() == self.capacity {
                if let Some(lru_node) = self.list.pop_tail() {
                    let lru_key = lru_node.borrow().key;
                    self.map.remove(&lru_key);
                }
            }
            let new_node = Node::new(key, value);
            self.list.push_front(new_node.clone());
            self.map.insert(key, new_node);
        }
    }
}
```

---

## ✅ Example Usage

```rust
fn main() {
    let mut lru = LRUCache::new(2);
    lru.put(1, 1);
    lru.put(2, 2);
    println!("{}", lru.get(1)); // 1
    lru.put(3, 3);
    println!("{}", lru.get(2)); // -1
    lru.put(4, 4);
    println!("{}", lru.get(1)); // -1
    println!("{}", lru.get(3)); // 3
    println!("{}", lru.get(4)); // 4
}
```

---

## 📝 Summary

This implementation shows how powerful Rust’s `Rc`, `RefCell`, and smart memory management can be when building classic data structures like LRU cache.