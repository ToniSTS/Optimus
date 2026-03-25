# Optimus Programming Language

A modern programming language built with Rust, designed to help developers write efficient code by providing real-time feedback on time and space complexity.

## Overview

Optimus is an educational programming language that analyzes code as you write it. Unlike traditional languages that only report syntax errors, Optimus determines the time and space complexity of your algorithms and provides actionable feedback on how to optimize them.

This makes Optimus particularly valuable for:

- **Students learning data structures and algorithms**
- **Developers preparing for technical interviews**
- **Anyone wanting to write more efficient code**

## Key Features

### Real-Time Complexity Analysis

Optimus automatically analyzes the time and space complexity of your code and provides feedback directly in the terminal, suggesting specific improvements.

### Clean, Readable Syntax

Optimus features a straightforward syntax inspired by modern languages:

```
// Calculate circle area
mut float pi = 3.14159;
float radius = 5.0;

mut float area = pi * radius * radius;
print(area);
```

### Mutability Control

Variables are immutable by default for safety. Use the `mut` keyword for variables that need to change:

```
mut int counter = 0;
counter = counter + 1;  // Allowed
int value = 5;
value = 10;  // Error: value is immutable
```

### Built-in Type System

- `int` - Integer values (64-bit)
- `float` - Floating-point numbers
- `bool` - Boolean values (`true` / `false`)
- `string` - Text strings

## Getting Started

### Prerequisites

- Rust toolchain (1.75 or later)

### Build from Source

```bash
cargo build --release
```

### Run the Compiler

```bash
cargo run
```

### Run Examples

```bash
# Build and run
cargo run --release

# Check for errors
cargo check
```

## Language Syntax

### Variables

```optimus
int x = 42;
mut float y = 3.14;
bool is_active = true;
string name = "Optimus";
```

### Arithmetic Operations

```optimus
int sum = 10 + 5;      // Addition
int diff = 10 - 5;     // Subtraction
int prod = 10 * 5;     // Multiplication
int quot = 10 / 5;     // Division
```

### Comparison Operators

```optimus
bool equal = 10 == 10;   // Equal
bool not_eq = 10 != 5;   // Not equal
bool greater = 10 > 5;   // Greater than
bool less = 5 < 10;      // Less than
```

### Print Output

```optimus
print("Hello, Optimus!");
print(42);
print(3.14);
```

### Control Flow (Coming Soon)

```optimus
// Conditionals
if (x > 0) {
    print(x);
} else {
    print("negative");
}

// Loops
for (int i = 0; i < 10; i = i + 1) {
    print(i);
}
```

## How Complexity Analysis Works

Optimus analyzes your code and detects common patterns that affect performance:

| Pattern                         | Complexity | Feedback                       |
| ------------------------------- | ---------- | ------------------------------ |
| Nested loops over the same data | O(n²)      | Consider using a hash map      |
| Repeated linear searches        | O(n)       | Pre-sort and use binary search |
| Unbounded recursion             | O(n)       | Consider iterative approach    |
| Large array allocations         | O(n) space | Reuse buffers when possible    |

Example feedback:

```
⚠️  Line 15: Nested loop detected (O(n²))
   → Consider using a hash map for O(n) lookup

✓  Line 22: Algorithm complexity is optimal (O(n log n))
```

## Roadmap

- [x] Lexer and tokenizer
- [x] Parser with operator precedence
- [x] Abstract Syntax Tree (AST) generation
- [x] Basic arithmetic expression evaluation
- [ ] Complexity analysis engine
- [ ] Type inference and checking
- [ ] If/else conditionals
- [ ] For and while loops
- [ ] Functions and user-defined procedures
- [ ] Standard library
- [ ] Code generation / interpretation

## Project Structure

```
optimus_compiler/
├── src/
│   ├── main.rs      # Entry point and demo
│   ├── lexer.rs     # Tokenizer (logos-based)
│   ├── parser.rs    # Parser (chumsky-based)
│   └── ast.rs       # AST node definitions
├── Cargo.toml       # Project manifest
└── README.md        # This file
```

## Contributing

Contributions are welcome! The project is in early development and there's plenty to work on.

## License

MIT
