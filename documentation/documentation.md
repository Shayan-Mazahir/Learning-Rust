# Rust Basics Documentation

## Table of Contents
1. [Essential Commands](#essential-commands)
2. [Variables and Constants](#variables-and-constants)
3. [Data Types](#data-types)
4. [Structs](#structs)
5. [Functions](#functions)
6. [Control Flow](#control-flow)
7. [Loops](#loops)

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

## Functions

Functions are sets of statements that perform a specific task. They are defined using the `fn` keyword.

### Basic Function Syntax

```rust
fn function_name() {
    // Function body
}
```

### Simple Function Example

```rust
fn simple_function() {
    println!("Me is a simple function");
}
```

### Functions with Parameters

Functions can accept parameters to work with different data.

```rust
fn two_parameter(num1: i32, num2: i32) {
    println!("I have 2 parameters");
    println!("The two numbers are: {num1} and {num2}. Their sum is: {}", num1 + num2);
}
```

### Statements vs Expressions

- **Statements:** Instructions that perform actions but don't return values (end with `;`)
- **Expressions:** Code that evaluates and returns a value (no `;` at the end)

```rust
fn another_function() {
    let x: i32 = 5;  // Statement
    let y: i32 = 6;  // Statement
    let z: i32 = x + y;  // Statement (x + y is an expression)
    println!("The value of z is: {z}");
}

fn expression_example() {
    let y: i32 = {
        let x: i32 = 3;
        x + 2  // Expression - no semicolon!
    };
    println!("The value of y is: {y}");
}
```

### Functions with Return Values

Functions can return values using the `->` syntax.

```rust
fn return_function(num1: i32, num2: i32) -> (i32, String) {
    let x = num1 + num2;
    let y = format!("The sum of the two numbers is: {}", x);
    (x, y)  // Return tuple - no semicolon!
}

// Usage
let result = return_function(10, 20);
println!("x: {}", result.0);
println!("y: {}", result.1);
```

**Function Return Rules:**
- Use `->` to specify return type
- Last expression is automatically returned (no `return` keyword needed)
- Don't use semicolon on the final expression
- Can return tuples for multiple values

---

## Control Flow

Control flow allows your program to make decisions and execute different code paths.

### If Expressions

```rust
let number = 3;

if number < 5 {
    println!("Number is less than 5");
} else if number > 5 {
    println!("Number is greater than 5");
} else {
    println!("Number is 5");
}
```

### If in Let Statements

Since `if` is an expression, you can use it in `let` statements.

```rust
let condition = true;
let number = if condition {
    5
} else {
    6
};
println!("The value of the number is {number}");
```

**Important:** Both branches must return the same type!

### Match Expressions

Match is a powerful pattern matching construct, similar to switch statements but more powerful.

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_coin(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

let coin = Coin::Penny;
println!("Value of coin is {}", value_in_coin(coin));
```

**Match Characteristics:**
- Must be exhaustive (cover all possibilities)
- More powerful than if/else for complex conditions
- Can match on enums, literals, ranges, and more

---

## Loops

Rust provides several types of loops for repetitive tasks.

### 1. Infinite Loop (`loop`)

```rust
loop {
    println!("This runs forever!");
    // Use 'break' to exit
}
```

### 2. While Loops

Loops that continue while a condition is true.

```rust
let mut i = 10;
while i != 0 {
    println!("{i}");
    i -= 1;
}
println!("DONE!");
```

### 3. For Loops

The most common loop type for iterating over collections.

#### Looping Through Arrays

```rust
let array = [1, 2, 3, 4, 5];

// Method 1: Using indices (not recommended)
let mut x: usize = 0;
while x < array.len() {
    println!("The value at index {x} is: {}", array[x]);
    x += 1;
}

// Method 2: Direct iteration (recommended) - secure way
for element in array {
    println!("The value is: {element}");
}
```

#### Range Loops

```rust
// Forward range
for number in (1..4) {
    println!("{number}");  // Prints 1, 2, 3
}

// Reverse range
for number in (1..4).rev() {
    println!("{number}");  // Prints 3, 2, 1
}
```

### 4. Loop with Return Values

Loops can return values when broken.

```rust
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter;  // Return counter value
    }
};
println!("The result is {result}");
```

### Loop Control

- `break` - Exit the loop immediately
- `continue` - Skip to the next iteration
- `break value` - Exit loop and return a value

**Best Practices:**
- Use `for` loops for iterating over collections
- Use `while` loops for condition-based repetition
- Use `loop` for infinite loops that break on specific conditions
- Avoid manual indexing when possible - use iterators instead

---