# Optimus Programming Language

A modern programming language built with Rust, designed to help developers write efficient code by providing real-time feedback on time and space complexity.

## Overview

Optimus is an educational programming language that analyzes your code and determines its time and space complexity, providing actionable feedback on how to optimize algorithms.

## Key Features

### Complexity Analysis Engine

Optimus executes your code while tracking time and space complexity, generating BIG-O complexity reports:

```
========================================
BIG-O COMPLEXITY REPORT
========================================
Time Complexity:  O(N)
Space Complexity: O(1)
========================================
```

### Full Control Flow

Support for conditionals and loops with automatic loop depth detection:

```optimus
// If/else statements
if (x > 0) {
    print("positive");
} else {
    print("negative");
}

// While loops
while (count < 10) {
    print(count);
    count = count + 1;
}

// For loops
for (mut int i = 0; i < 10; i = i + 1) {
    print(i);
}
```

### Clean, Readable Syntax

```optimus
mut float pi = 3.14159;
float radius = 5.0;

mut float area = pi * radius * radius;
print(area);
```

### Mutability Control

Variables are immutable by default. Use `mut` for variables that need to change:

```optimus
mut int counter = 0;
counter = counter + 1;  // Allowed
int value = 5;
value = 10;  // Error: value is immutable
```

### Full Type System

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

### Run a Program

```bash
cargo run -- examples/hello.op
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

### Control Flow

```optimus
if (x > 0) {
    print(x);
} else {
    print("negative");
}

for (mut int i = 0; i < 10; i = i + 1) {
    print(i);
}

while (count < 10) {
    count = count + 1;
}
```

## Complexity Analysis

Optimus analyzes loop nesting depth to determine complexity:

| Loop Depth | Complexity |
| ---------- | ---------- |
| 0          | O(1)       |
| 1          | O(N)       |
| 2          | O(N²)      |
| 3+         | O(N^X)     |

Example output:

```
========================================
BIG-O COMPLEXITY REPORT
========================================
Time Complexity:  O(N^2)
Space Complexity: O(1)
========================================
```

## Roadmap

- [x] Lexer and tokenizer (logos-based)
- [x] Parser with operator precedence (chumsky-based)
- [x] Abstract Syntax Tree (AST) generation
- [x] Arithmetic expression evaluation
- [x] Complexity analysis engine
- [x] If/else conditionals
- [x] For and while loops
- [ ] Type inference
- [ ] Functions and user-defined procedures
- [ ] Standard library
- [ ] Bytecode code generation

## Project Structure

```
optimus_compiler/
├── src/
│   ├── main.rs      # Entry point
│   ├── lexer.rs     # Tokenizer (logos-based)
│   ├── parser.rs    # Parser (chumsky-based)
│   ├── ast.rs       # AST node definitions
│   └── analyzer.rs  # Execution & complexity analysis
├── Cargo.toml       # Project manifest
└── README.md        # This file
```

## Contributing

Contributions are welcome! The project is in active development.

## License

MIT
