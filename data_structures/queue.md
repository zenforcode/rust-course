
## ⭐ Counting Islands Using BFS in Rust

### 🔢 Problem Statement

Given a 2D grid of `'1'`s (land) and `'0'`s (water), count the number of **islands**. An island is formed by connecting adjacent lands **horizontally or vertically**. You may assume all four edges of the grid are surrounded by water.

### 👉 Example:

**Input:**
```
[
  ['1','1','0','0','0'],
  ['1','1','0','0','0'],
  ['0','0','1','0','0'],
  ['0','0','0','1','1']
]
```

**Output:** `3`

---

### 🤖 Rust Solution (BFS)

```rust
use std::collections::VecDeque;

fn num_islands_bfs(grid: &mut Vec<Vec<char>>) -> i32 {
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }
    let cols = grid[0].len();
    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == '1' {
                count += 1;
                bfs(grid, row, col);
            }
        }
    }

    count
}

fn bfs(grid: &mut Vec<Vec<char>>, start_row: usize, start_col: usize) {
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut queue = VecDeque::new();
    queue.push_back((start_row, start_col));
    grid[start_row][start_col] = '0';

    while let Some((row, col)) = queue.pop_front() {
        for (dr, dc) in directions.iter() {
            let new_row = row as isize + dr;
            let new_col = col as isize + dc;
            if new_row >= 0
                && new_col >= 0
                && new_row < grid.len() as isize
                && new_col < grid[0].len() as isize
                && grid[new_row as usize][new_col as usize] == '1'
            {
                grid[new_row as usize][new_col as usize] = '0';
                queue.push_back((new_row as usize, new_col as usize));
            }
        }
    }
}

fn main() {
    let mut grid = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ];

    let result = num_islands_bfs(&mut grid);
    println!("Number of islands: {}", result);
}
```

---

### 📝 Notes:
- This version uses a **queue (BFS)** instead of recursion.
- Safe for large grids (avoids stack overflow).
- BFS floods connected land horizontally and vertically, marking visited cells as visited (`'0'`).
