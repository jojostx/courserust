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
## Development Container Configuration

This repository includes a `devcontainer.json` file to set up a development container for Rust. The configuration uses the Rust image from Microsoft's container registry and includes customizations for Visual Studio Code.

### Key Features

- **Image**: The container uses the `mcr.microsoft.com/devcontainers/rust:0-1-bullseye` image.
- **VS Code Extensions**: The container is pre-configured with the following extensions:
    - `rust-lang.rust-analyzer` for Rust language support.
    - `vadimcn.vscode-lldb` for debugging.
    - `GitHub.copilot` for code generation and autocompletion.

### Additional Configuration Options

- **Persistent Cargo Cache**: Uncomment the `mounts` section to make the cargo cache persistent in a Docker Volume.
- **Post Create Command**: The `postCreateCommand` is set to check the Rust compiler version.
- **Forward Ports**: You can specify ports to forward from the container to your local machine.
- **Root Access**: Uncomment the `remoteUser` section to connect as root.

This setup ensures a consistent development environment, making it easier to manage dependencies and tools.
