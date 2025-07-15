# Rust Basics Documentation

## Table of Contents
1. [Essential Commands](#essential-commands)
2. [Variables and Constants](#variables-and-constants)
3. [Data Types](#data-types)
4. [Structs](#structs)

---

## Essential Commands

### Project Management
- `cargo new <project_name>` - Initialize a new Rust project
- `cargo build` - Build/compile the entire project
- `cargo check` - Check if code can compile without actually compiling
- `cargo run` - Run the program

### File Operations
- `touch <filename>.rs` - Create a new Rust file
- `rustc <filename>.rs` - Compile a single Rust file

---

## Variables and Constants

### 1. Variable Mutability

By default, Rust variables are **immutable** (cannot be changed). This is a security feature that prevents accidental modifications.

```rust
let x = 5;        // Immutable - cannot change value
let mut y = 10;   // Mutable - can change value
```

**Key Points:**
- `let` creates immutable variables by default
- `let mut` creates mutable variables
- With immutable variables, only the value can be changed, not the type
- Mutability allows changing both value and type

### 2. Variable Shadowing

Each `let` statement creates a new variable, even with the same name. This allows you to reuse variable names with different data types.

```rust
let password = 123;           // Integer
let password = "password123"; // String
let password = 987;           // Integer again
// All are different variables with the same name
```

**Benefits:**
- Reuse meaningful variable names
- Transform data types while keeping the same name
- Each shadowed variable is still immutable (unless declared with `mut`)

### 3. Variable Scope

Variables exist only within their defined scope (code block).

```rust
let user_role = "USER";           // Outer scope
{
    let user_role = "ADMIN";      // Inner scope (shadows outer)
    let temp_var = "SECRET";      // Only exists in this block
}
// user_role is back to "USER"
// temp_var doesn't exist here!
```

**Scope Rules:**
- Inner scopes can shadow outer variables
- When inner scope ends, shadowing disappears
- Variables only live within their scope

### 4. Constants

Constants are values that are immutable forever and must be known at compile time.

```rust
const MAX_LOGIN_ATTEMPTS: u32 = 3;
const ADMIN_PRIVILEGES: &str = "ADMIN";
```

**Constant Characteristics:**
- **Immutable forever:** Cannot be changed, ever
- **Compile-time values:** Must be calculable at compile time
- **Global scope:** Available throughout the program
- **Type required:** Must explicitly specify the type
- **Naming convention:** SCREAMING_SNAKE_CASE

---

## Data Types

Rust has three main categories of data types:

### 1. Scalar Types

#### Integers
| Length  | Signed (negative + positive) | Unsigned (positive only) |
|---------|------------------------------|--------------------------|
| 8-bit   | i8                          | u8                       |
| 16-bit  | i16                         | u16                      |
| 32-bit  | i32                         | u32                      |
| 64-bit  | i64                         | u64                      |
| 128-bit | i128                        | u128                     |
| arch    | isize                       | usize                    |

**Integer Literals:**
```rust
let decimal = 9820;
let hex = 0xff;        // Hexadecimal
let octal = 0o77;      // Octal
let binary = 0b1010101; // Binary
let byte = b'A';       // Byte (u8 only)
```

#### Floating-Point Numbers
```rust
let x: f32 = 2.5;  // 32-bit float
let y = 2.0;       // 64-bit float (default)
```

**Arithmetic Operations:**
```rust
println!("Addition: {}", x + y);
println!("Division: {}", x / y);
println!("Remainder: {}", x % y);
```

### 2. Compound Types

#### Tuples
Tuples group multiple values with different types.

```rust
let tup: (i32, f64, char) = (500, 6.4, 'x');

// Destructuring
let (x, y, z) = tup;
println!("The value of y is {}", y);

// Direct access
println!("The value of 500 is: {}", tup.0);
```

#### Arrays
Arrays store multiple values of the same type.

```rust
let arr = [1, 2, 3, 4, 5];  // Array of 5 elements
println!("First value: {}", arr[0]);
```

### 3. Custom Types

Custom types are user-defined data structures.

---

## Structs

Structs allow you to create custom data types that group related data together.

### Basic Struct Definition

```rust
#[derive(Debug)]  // Enables debug printing
struct Person {
    name: String,
    age: u8,
}
```

### Creating and Using Structs

```rust
let person = Person {
    name: String::from("John"),
    age: 25,
};

// Debug print (requires #[derive(Debug)])
println!("Person is {:?}", person);
```

### Why `#[derive(Debug)]`?

The `#[derive(Debug)]` attribute automatically implements the `Debug` trait for your struct, which:
- Enables printing with `{:?}` and `{:#?}` format specifiers 
- Provides a default debug representation
- Is essential for debugging and development
- **Without it, you cannot print the struct directly**

---
