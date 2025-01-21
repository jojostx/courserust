# courserust
repo containing learning and practice exercises for my coursera rust course

# Rust Setup

This repository contains a simple Rust program that calculates the factorial of a given number.

## Function Description

### `factorial`

The `factorial` function takes an unsigned 32-bit integer `n` as input and returns the factorial of `n`. The function uses recursion to compute the factorial. The base cases are when `n` is 0 or 1, in which case the function returns 1. For all other values of `n`, the function returns `n` multiplied by the factorial of `n - 1`.

```rust
fn factorial(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}
```

### `main`

The `main` function demonstrates the usage of the `factorial` function by calculating the factorial of 5 and printing the result.

```rust
fn main() {
    let num = 5;
    println!("The factorial of {} is {}", num, factorial(num));
}
```

I found the code generation and autocompletion of Copilot useful.
