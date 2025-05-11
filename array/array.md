Programming Assignment: Maximum Hourglass Sum in Rust
Objective

Write a function in Rust that computes the maximum hourglass sum in a 6x6 2D array. An hourglass is a subset of values with indices following this pattern:

a b c
  d
e f g

You must identify all possible hourglasses in the array and return the highest sum found among them.
Function Signature

fn max_hourglass_sum(arr: [[i32; 6]; 6]) -> i32;

Input

    A 6x6 array arr, where each element is an integer in the range -9 <= arr[i][j] <= 9.

Output

    Return the maximum hourglass sum as a 32-bit signed integer.

Example
Input

let arr = [
    [2, 1, 1, 0, 0, 0],
    [0, 2, 0, 0, 0, 0],
    [1, 1, 2, 0, 0, 0],
    [0, 0, 3, 4, 4, 0],
    [0, 0, 0, 3, 0, 0],
    [0, 0, 1, 2, 4, 0],
];

Output

19

Explanation

The hourglass with the highest sum is:

3 4 4
  3
1 2 4

Which gives: 3 + 4 + 4 + 3 + 1 + 2 + 4 = 19
Constraints

    The array is always 6x6.

    Each value is an integer from -9 to 9.
