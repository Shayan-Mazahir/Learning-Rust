# Module 01: Introduction to Rust
## Systems Programming Language
### University-Style Learning Guide

---

## Computer Science is Not Programming

Just as with functional programming, systems programming is about understanding computation at a fundamental level. Rust provides a concrete medium to explore memory safety, ownership, and concurrent programming.

> "Rust is no more about low-level programming than chemistry is about test tubes."

---

## Core Themes in Rust

### 1. **Memory Safety Without Garbage Collection**
- Code must be safe by construction
- The compiler enforces rules that prevent entire classes of bugs
- Zero-cost abstractions: safety doesn't mean slow

### 2. **Ownership and Borrowing**
- Every value has exactly one owner
- Values can be borrowed (referenced) under strict rules
- The compiler tracks lifetimes automatically

### 3. **Explicitness Over Implicitness**
- Rust makes you think about error handling
- Type conversions are explicit
- Mutability must be declared

### 4. **Fearless Concurrency**
- The type system prevents data races at compile time
- Safe sharing of data between threads
- Performance without compromising safety

---

## Why Rust?

- **Memory-safe** by default
- **Performance** comparable to C/C++
- **Modern** tooling and package management
- **Expressive** type system
- **Growing** ecosystem
- Notoriously **picky compiler** (but that's a feature!)

The Rust compiler is like a strict but caring teacher - it catches your mistakes before they become runtime bugs.

---

## The Language of Programming

Just as mathematics has its notation, Rust has precise syntax and semantics.

### Values in Rust

```rust
// Integer types
0          // i32 by default
-37        // i32
42_u8      // unsigned 8-bit integer
1000_i64   // signed 64-bit integer

// Floating point
3.14159    // f64 by default
2.5_f32    // 32-bit float

// Booleans
true
false

// Characters and strings
'a'        // char (Unicode scalar value)
"hello"    // &str (string slice)

// Tuples
(6, 4)
(true, 3.14, 'x')

// Arrays (fixed size)
[1, 2, 3, 4, 5]
```

### Expressions vs Statements

**In Rust, almost everything is an expression** (produces a value).

```rust
// Expression (returns a value)
5 + 3          // evaluates to 8

// Statement (doesn't return a useful value)
let x = 5;     // variable binding

// Block expression (last expression is the value)
{
    let a = 5;
    let b = 10;
    a + b      // no semicolon - this is returned
}              // evaluates to 15
```

---

## 🎯 Concept Check 1: Understanding Values

### Quick Questions

1. **What's the difference between `42` and `42_u8` in Rust?**
   
   <details>
   <summary>Click to reveal answer</summary>
   
   `42` is an `i32` by default (signed 32-bit integer), while `42_u8` is explicitly an unsigned 8-bit integer. The underscore and type suffix force a specific type.
   </details>

2. **Why does Rust distinguish between `'a'` (char) and `"a"` (string slice)?**
   
   <details>
   <summary>Click to reveal answer</summary>
   
   `'a'` is a single Unicode scalar value (4 bytes), while `"a"` is a string slice (a reference to a sequence of UTF-8 bytes). They have completely different memory representations and use cases.
   </details>

3. **Written Response: Explain why `(1, 2, 3)` and `[1, 2, 3]` are fundamentally different in Rust.**
   
   <details>
   <summary>Click to reveal answer</summary>
   
   `(1, 2, 3)` is a tuple - it can hold different types and is accessed by position (`.0`, `.1`, `.2`). `[1, 2, 3]` is an array - all elements must be the same type, it has a fixed size, and elements are accessed by index. Tuples are heterogeneous, arrays are homogeneous.
   </details>

---

## Rust Arithmetic and Operators

### Basic Arithmetic

Unlike Racket's prefix notation `(+ 5 3)`, Rust uses **infix notation**:

```rust
// Addition
5 + 3          // 8

// Subtraction  
10 - 4         // 6

// Multiplication
6 * 7          // 42

// Division (integer division if both operands are integers)
20 / 3         // 6 (truncates)
20.0 / 3.0     // 6.666666666666667

// Remainder (modulo)
17 % 5         // 2

// Exponentiation (requires method call)
2_i32.pow(8)   // 256
```

### Operator Precedence

Rust follows standard mathematical precedence (like PEMDAS):

```rust
5 * 6 + 7      // 37 (multiplication first)
5 * (6 + 7)    // 65 (parentheses override)

// No ambiguity needed - precedence is well-defined
```

### Important Type Considerations

**Rust is STRICT about types:**

```rust
// This FAILS - cannot mix integer and float
let x = 5 + 2.5;  // ❌ ERROR: mismatched types

// You must explicitly cast
let x = 5.0 + 2.5;           // ✅ Both f64
let x = 5 as f64 + 2.5;      // ✅ Explicit conversion
let x = (5 as f64) + 2.5;    // ✅ Extra clear
```

### Useful Math Operations

```rust
// Absolute value
(-5_i32).abs()              // 5

// Square root (only for floats)
25.0_f64.sqrt()             // 5.0

// Power
2_i32.pow(10)               // 1024
2.0_f64.powf(10.0)          // 1024.0 (float version)

// Min and max
i32::min(5, 10)             // 5
i32::max(5, 10)             // 10

// Trigonometry (need to use std::f64::consts)
use std::f64::consts::PI;
PI.sin()                    // approximately 0
(PI / 2.0).cos()            // approximately 0
```

---

## 📝 Quiz 1: Arithmetic and Type Strictness

### Multiple Choice

**Q1: What is the result of `20 / 3` in Rust (assuming both are i32)?**
- A) 6.666...
- B) 6
- C) 7
- D) Compilation error

<details>
<summary>Answer</summary>

**B) 6** - Integer division truncates towards zero. Both operands are integers, so the result is an integer.
</details>

**Q2: Which of the following will compile successfully?**
- A) `let x = 5 + 2.5;`
- B) `let x = 5.0 + 2.5;`
- C) `let x: i32 = 5 + 2;`
- D) Both B and C

<details>
<summary>Answer</summary>

**D) Both B and C** 
- A fails because you can't mix integer and float types
- B works because both are floats (5.0 is f64)
- C works because both are integers
</details>

### Written Response Questions

**Q3: Debug this code. What's wrong and how would you fix it?**
```rust
let result = 10 / 4.0;
```

<details>
<summary>Answer</summary>

**Problem:** Type mismatch - `10` is an integer (i32) and `4.0` is a float (f64).

**Fixes:**
```rust
// Option 1: Make both floats
let result = 10.0 / 4.0;

// Option 2: Cast the integer
let result = (10 as f64) / 4.0;

// Option 3: Explicit type annotation
let result: f64 = 10.0 / 4.0;
```
</details>

**Q4: Explain why `2_i32.pow(8)` requires the method syntax instead of a simple operator like `2 ^ 8`.**

<details>
<summary>Answer</summary>

**Reason:** In Rust, `^` is the XOR (exclusive or) bitwise operator, not exponentiation. To avoid confusion and maintain consistency with other languages' bitwise operators, Rust requires explicit method calls for power operations. This makes the operation unambiguous and type-safe, as different numeric types have different `pow` implementations (`pow` for integers takes `u32`, while `powf` for floats takes the same type).
</details>

**Q5: Trace through this expression step by step: `(10 + 5 * 2) / 4`**

<details>
<summary>Answer</summary>

```
(10 + 5 * 2) / 4
→ (10 + 10) / 4      [multiplication first: 5 * 2]
→ 20 / 4              [parentheses: 10 + 10]
→ 5                   [division]
```

Note: The result is 5, not 5.0, because all operands are integers, performing integer division.
</details>

---

## Variables and Constants

### Immutable Bindings (Default)

In Rust, **variables are immutable by default** - this is a key difference from most languages:

```rust
let x = 5;
// x = 6;  // ❌ ERROR: cannot assign twice to immutable variable
```

This is similar to how mathematical constants work: once you define `a = 13`, it stays 13.

### Mutable Bindings

You must **explicitly** declare mutability:

```rust
let mut x = 5;
x = 6;      // ✅ OK because x is mutable
x = x + 1;  // ✅ Can update mutable variables
```

### Constants (Compile-Time Values)

True constants are declared with `const` and **must** have a type annotation:

```rust
const PI: f64 = 3.14159265359;
const MAX_POINTS: u32 = 100_000;
const AVOGADRO: f64 = 6.02214e23;

// Constants can be used anywhere
const EULER: f64 = 2.718281828;
const GOLDEN_RATIO: f64 = (1.0 + 5.0_f64.sqrt()) / 2.0;
```

**Key Differences:**
- `const`: Compile-time constant, must be calculable at compile time
- `let`: Runtime binding, can be calculated from runtime values
- `let mut`: Mutable runtime binding

### Naming Conventions

```rust
// Constants: SCREAMING_SNAKE_CASE
const SPEED_OF_LIGHT: f64 = 299_792_458.0;

// Variables and functions: snake_case
let student_name = "Ruz";
let cs_grade = 95.73;

// Types and traits: PascalCase (UpperCamelCase)
struct StudentRecord { }
enum GradeLevel { }
```

---

## 🎯 Concept Check 2: Mutability and Constants

### Quick Questions

**Q1: What's the output of this code?**
```rust
let x = 5;
let x = x + 1;
let x = x * 2;
println!("{}", x);
```

<details>
<summary>Answer</summary>

**12** - This uses shadowing:
- First x = 5
- Second x = 5 + 1 = 6 (shadows first)
- Third x = 6 * 2 = 12 (shadows second)
</details>

**Q2: Will this compile? Why or why not?**
```rust
const MY_NUMBER = 42;
```

<details>
<summary>Answer</summary>

**No** - Constants must have explicit type annotations:
```rust
const MY_NUMBER: i32 = 42;  // ✅ Correct
```
</details>

### Written Response Questions

**Q3: Explain the difference between `let` and `const`. Give scenarios where you'd use each.**

<details>
<summary>Answer</summary>

**`const`:**
- Compile-time constant
- Must have type annotation
- Value must be computable at compile time
- Cannot be shadowed (in the same scope)
- Use for: Mathematical constants (PI, E), configuration values, magic numbers

**`let`:**
- Runtime binding
- Type can be inferred
- Value can be computed at runtime
- Can be shadowed
- Use for: Function results, user input, calculations

**Examples:**
```rust
const MAX_POINTS: u32 = 100_000;  // Configuration
let user_score = calculate_score();  // Runtime value
```
</details>

**Q4: Debug this code and explain what's wrong:**
```rust
let message = "Hello";
message = "Goodbye";
println!("{}", message);
```

<details>
<summary>Answer</summary>

**Problem:** Attempting to reassign to an immutable variable.

**Fix Option 1 - Make it mutable:**
```rust
let mut message = "Hello";
message = "Goodbye";
println!("{}", message);
```

**Fix Option 2 - Use shadowing:**
```rust
let message = "Hello";
let message = "Goodbye";  // Creates new binding
println!("{}", message);
```

**When to use which:** Use `mut` when you're truly modifying the same variable in a loop or accumulator pattern. Use shadowing when transforming a value or changing its type.
</details>

**Q5: Why is shadowing useful? Give a concrete example where it's better than using `mut`.**

<details>
<summary>Answer</summary>

**Shadowing is useful because it allows type changes:**

```rust
// With shadowing (✅ works)
let spaces = "   ";           // &str type
let spaces = spaces.len();    // usize type

// With mut (❌ doesn't work)
let mut spaces = "   ";       
// spaces = spaces.len();     // ERROR: can't change types

// Practical use case:
let input = "42";                    // String input
let input = input.trim();            // Trimmed string
let input: i32 = input.parse()       // Parsed integer
    .expect("Not a number");
```

Each step transforms the data, and shadowing makes it clear we're working with the "same conceptual value" in different forms.
</details>

---

## Functions in Rust

### Basic Function Syntax

Unlike Racket's `(define (f x) ...)`, Rust uses a more verbose but explicit syntax:

```rust
// Mathematical: f(x) = x² - x - 1
fn f(x: i32) -> i32 {
    x * x - x - 1
}

// Racket: (define (f x) (- (- (sqr x) x) 1))
```

### Anatomy of a Function

```rust
fn function_name(parameter: Type) -> ReturnType {
    // function body
    expression  // returned (no semicolon!)
}
```

**Breaking it down:**

1. `fn` - keyword declaring a function
2. `function_name` - identifier (snake_case by convention)
3. `(parameter: Type)` - parameters with **mandatory** type annotations
4. `-> ReturnType` - explicit return type
5. `{ }` - function body
6. Last expression without `;` is returned

### The Return Mechanism

```rust
// Implicit return (Rust style)
fn add(a: i32, b: i32) -> i32 {
    a + b  // no semicolon - this is returned
}

// Explicit return (also valid)
fn add_explicit(a: i32, b: i32) -> i32 {
    return a + b;  // early return or explicit
}

// With semicolon - returns unit type ()
fn does_nothing(a: i32, b: i32) -> () {
    a + b;  // semicolon means "discard this value"
}           // implicitly returns ()
```

**Critical Rule:** The last expression in a function block (without a semicolon) is the return value.

### Multiple Parameters

```rust
// Law of cosines: c² = a² + b² - 2ab·cos(γ)
fn law_of_cosines(a: f64, b: f64, gamma: f64) -> f64 {
    let a_squared = a * a;
    let b_squared = b * b;
    let cos_term = 2.0 * a * b * gamma.cos();
    
    (a_squared + b_squared - cos_term).sqrt()
}

// Call it
let side = law_of_cosines(5.0, 7.0, 1.047);  // ~60 degrees in radians
```

### Type Annotations are MANDATORY

```rust
// ❌ This fails - Rust can't infer parameter types
fn broken(x) -> i32 {
    x * 2
}

// ✅ Must specify types
fn working(x: i32) -> i32 {
    x * 2
}

// ✅ Return type can sometimes be inferred, but be explicit
fn inferred_return(x: i32) {
    x * 2  // Compiler knows this returns i32
}
```

---

## 📝 Quiz 2: Function Fundamentals

### Multiple Choice

**Q1: What does this function return?**
```rust
fn mystery(x: i32) -> i32 {
    x + 5;
}
```
- A) `x + 5`
- B) `()`
- C) Compilation error
- D) `5`

<details>
<summary>Answer</summary>

**C) Compilation error** - The function signature says it returns `i32`, but the semicolon after `x + 5` makes it a statement that returns `()`. Remove the semicolon or add an explicit `return`.
</details>

**Q2: Which function definition is valid?**
- A) `fn calc(x) -> i32 { x * 2 }`
- B) `fn calc(x: i32) { x * 2 }`
- C) `fn calc(x: i32) -> i32 { x * 2 }`
- D) `fn calc(x: i32): i32 { x * 2 }`

<details>
<summary>Answer</summary>

**C)** is the only valid one:
- A: Missing parameter type
- B: Missing return type (would need to return `()`)
- C: ✅ Correct
- D: Wrong syntax (uses `:` instead of `->`)
</details>

### Debugging Challenges

**Q3: Fix all the errors in this function:**
```rust
fn calculate_average(a, b: i32) -> f64 {
    let sum = a + b;
    sum / 2
}
```

<details>
<summary>Answer</summary>

**Problems:**
1. Parameter `a` is missing a type
2. Integer division will truncate (need float division)
3. Type mismatch: returns i32 but signature says f64

**Fixed version:**
```rust
fn calculate_average(a: i32, b: i32) -> f64 {
    let sum = a + b;
    sum as f64 / 2.0  // Convert to f64 and use float division
}

// Alternative:
fn calculate_average(a: i32, b: i32) -> f64 {
    (a as f64 + b as f64) / 2.0
}
```
</details>

### Written Response Questions

**Q4: Explain the difference between these two functions. When would each be appropriate?**
```rust
fn version1(x: i32) -> i32 {
    return x * 2;
}

fn version2(x: i32) -> i32 {
    x * 2
}
```

<details>
<summary>Answer</summary>

**Both are functionally equivalent**, but:

**`version1` (explicit return):**
- Uses the `return` keyword
- Useful for early returns or complex control flow
- More familiar to programmers from C/Java/Python

**`version2` (implicit return):**
- Rust idiomatic style
- Cleaner for simple functions
- Last expression without semicolon is returned
- Preferred in Rust community

**Use explicit `return` when:**
```rust
fn process(x: i32) -> i32 {
    if x < 0 {
        return 0;  // Early return
    }
    x * 2  // Implicit return at end
}
```
</details>

**Q5: Write a function `distance` that takes four parameters (x1, y1, x2, y2) and returns the Euclidean distance between two points: √((x₂-x₁)² + (y₂-y₁)²)**

<details>
<summary>Answer</summary>

```rust
fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    (dx * dx + dy * dy).sqrt()
}

// Alternative (more compact):
fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

// Note: We use f64 for parameters to support fractional coordinates
// and because sqrt() is only available on floats
```
</details>

**Q6: What's wrong with this approach? How would you fix it?**
```rust
fn circle_area(radius) {
    3.14159 * radius * radius
}
```

<details>
<summary>Answer</summary>

**Problems:**
1. Missing parameter type annotation
2. Missing return type annotation
3. Using hardcoded π instead of std constant

**Fixed version:**
```rust
fn circle_area(radius: f64) -> f64 {
    std::f64::consts::PI * radius * radius
}

// Even better with powi:
fn circle_area(radius: f64) -> f64 {
    std::f64::consts::PI * radius.powi(2)
}
```
</details>

---

## Function Definition Grammar

### General Form

```rust
fn identifier(param1: Type1, param2: Type2, ...) -> ReturnType {
    // function body
    expression
}
```

### Rules

1. **Function name:** Must be a valid identifier (snake_case convention)
2. **Parameters:** Zero or more, each with explicit type
3. **Return type:** Specified after `->`, or omit for `()` (unit type)
4. **Body:** Block containing expressions and statements
5. **Return value:** Last expression (no semicolon) or explicit `return`

### Examples with Increasing Complexity

```rust
// No parameters, no meaningful return
fn greet() {
    println!("Hello, Waterloo!");
}

// One parameter, return value
fn square(x: i32) -> i32 {
    x * x
}

// Multiple parameters
fn area_of_rectangle(width: f64, height: f64) -> f64 {
    width * height
}

// Multiple statements, final expression returned
fn geometric_series(r: f64, n: u32) -> f64 {
    let numerator = r.powf(n as f64) - 1.0;
    let denominator = r - 1.0;
    numerator / denominator
}

// Complex calculation
fn quadratic(a: f64, b: f64, c: f64, x: f64) -> f64 {
    a * x * x + b * x + c
}
```

---

## Scope and Shadowing

### Kinds of Scopes

Rust has several scope types:

#### 1. **Global Scope** (Module Level)

```rust
// Available everywhere in the module
const GLOBAL_CONSTANT: i32 = 42;

fn function1() {
    println!("{}", GLOBAL_CONSTANT);  // ✅ Can access
}

fn function2() {
    println!("{}", GLOBAL_CONSTANT);  // ✅ Can access
}
```

#### 2. **Function Scope**

```rust
fn calculate(x: i32) -> i32 {
    let y = x * 2;  // y only exists in this function
    y + 10
}
// println!("{}", y);  // ❌ ERROR: y doesn't exist here
```

#### 3. **Block Scope**

```rust
fn main() {
    let x = 5;
    
    {
        let y = 10;
        println!("Inner: x={}, y={}", x, y);  // ✅ Both accessible
    }
    
    println!("Outer: x={}", x);   // ✅ x still accessible
    // println!("y={}", y);        // ❌ ERROR: y no longer exists
}
```

### Shadowing (A Rust Superpower)

Unlike most languages, Rust allows **shadowing** - redefining a variable:

```rust
fn main() {
    let x = 5;
    
    let x = x + 1;  // ✅ Shadows previous x
    
    let x = x * 2;  // ✅ Shadows again
    
    println!("x = {}", x);  // Prints: x = 12
}
```

**Why shadowing is useful:**

```rust
// Type transformation
let spaces = "   ";           // &str
let spaces = spaces.len();    // usize - different type!

// Cannot do this with mut:
let mut spaces = "   ";
// spaces = spaces.len();     // ❌ ERROR: type mismatch
```

### Scope Rules (The Rust Commandments)

#### Rule 1: Variables Live Only in Their Scope

```rust
fn outer() {
    let a = 5;
    
    fn inner() {
        // println!("{}", a);  // ❌ ERROR: can't access outer function's variables
    }
}
```

#### Rule 2: Inner Scopes Can Shadow Outer Scopes

```rust
let x = 5;

{
    let x = 10;         // ✅ Shadows outer x
    println!("{}", x);  // Prints: 10
}

println!("{}", x);      // Prints: 5 (outer x)
```

#### Rule 3: Parameters Shadow Global Names

```rust
const VALUE: i32 = 100;

fn calculate(VALUE: i32) -> i32 {  // ✅ Parameter shadows constant
    VALUE * 2                       // Uses parameter, not constant
}

println!("{}", calculate(5));       // Prints: 10
println!("{}", VALUE);              // Prints: 100
```

---

## 🎯 Concept Check 3: Scope and Shadowing

### Scope Analysis Questions

**Q1: What does this print?**
```rust
let x = 10;
{
    let x = 20;
    println!("Inner: {}", x);
}
println!("Outer: {}", x);
```

<details>
<summary>Answer</summary>

```
Inner: 20
Outer: 10
```

The inner block shadows `x` with value 20, but this shadow only exists within the block. After the block ends, the outer `x` (value 10) is still in scope.
</details>

**Q2: Will this compile? Trace through what happens:**
```rust
const MULTIPLIER: i32 = 5;

fn calculate(MULTIPLIER: i32) -> i32 {
    MULTIPLIER * 2
}

fn main() {
    println!("{}", calculate(10));
    println!("{}", MULTIPLIER);
}
```

<details>
<summary>Answer</summary>

**Yes, this compiles.** Output:
```
20
5
```

Inside `calculate`, the parameter `MULTIPLIER` shadows the constant. The function uses the parameter value (10), so `10 * 2 = 20`. In `main`, the constant `MULTIPLIER` is still 5.
</details>

### Written Response Questions

**Q3: Explain why Rust allows parameter names to shadow global names. What are the benefits and potential pitfalls?**

<details>
<summary>Answer</summary>

**Benefits:**
- **Flexibility:** You can use descriptive names without worrying about conflicts
- **Local reasoning:** Function parameters are self-contained
- **Refactoring safety:** Adding a global constant won't break existing functions

**Pitfalls:**
- **Confusion:** Can be unclear which value you're using
- **Unintended shadowing:** Might shadow a global you actually wanted to use

**Example pitfall:**
```rust
const MAX_SIZE: usize = 100;

fn process(MAX_SIZE: usize) {
    // Programmer might think they're using the constant,
    // but actually using the parameter
    let buffer = vec![0; MAX_SIZE];
}
```

**Best practice:** Use different names when possible to avoid confusion.
</details>

**Q4: Debug this code - identify all scope-related errors:**
```rust
fn outer() {
    let x = 5;
    
    fn inner() {
        let y = x + 10;  // Line A
        println!("{}", y);
    }
    
    inner();
    println!("{}", y);  // Line B
}
```

<details>
<summary>Answer</summary>

**Errors:**

**Line A:** `x` is not accessible in `inner()`. In Rust, nested functions cannot capture variables from their enclosing function scope (unlike closures).

**Line B:** `y` is not accessible here - it only exists inside the `inner()` function.

**Fixed version (using closure instead):**
```rust
fn outer() {
    let x = 5;
    
    let inner = || {  // Closure can capture x
        let y = x + 10;
        println!("{}", y);
        y
    };
    
    let result = inner();
    println!("{}", result);  // Use returned value
}
```
</details>

---

## Scope Trials (Practice Problems)

Determine if each program is **valid** or **invalid**, and what it would output if valid:

### 1.
```rust
fn test1(x: i32, y: i32) -> i32 {
    x * 4
}

fn test2(x: i32, y: i32) -> i32 {
    y * 3
}
```
**Answer:** ✅ Valid - functions can have unused parameters

---

### 2.
```rust
let name = 12;
fn name(y: i32) -> i32 {
    y * y + 1
}
```
**Answer:** ❌ Invalid - cannot have a variable and function with the same name in the same scope

---

### 3.
```rust
fn s(t: i32) -> i32 {
    t + 3
}

fn t(s: i32) -> i32 {
    s + 4
}
```
**Answer:** ✅ Valid - parameter names can match other function names (different scopes)

---

### 4.
```rust
fn a(sub: i32, mul: i32, div: i32) -> i32 {
    sub + mul + div
}
```
**Answer:** ✅ Valid - parameter names can be anything (even if they match operation names conceptually)

---

### 5.
```rust
fn func(x: i32, x: i32) -> i32 {
    x + x * x
}
```
**Answer:** ❌ Invalid - cannot have duplicate parameter names

---

### 6.
```rust
let a = b - 5;
let b = 17;
```
**Answer:** ❌ Invalid - cannot use `b` before it's defined (Rust enforces declaration order)

---

### 7.
```rust
fn g(a: i32) -> i32 {
    a + P
}

const P: i32 = 13;
```
**Answer:** ✅ Valid - constants can be used before their definition (they're compile-time)

---

### 8.
```rust
fn z(z: i32) -> i32 {
    5 * z * 5
}
```
**Answer:** ✅ Valid - parameter name can shadow function name within the function

---

## Semantics and Evaluation

### How Rust Evaluates Code

Unlike Racket's substitution model, Rust uses a more complex evaluation strategy involving:

1. **Type checking** (at compile time)
2. **Borrow checking** (at compile time)  
3. **Code generation** (compilation)
4. **Execution** (runtime)

But we can think about evaluation conceptually:

### Evaluating Expressions

```rust
// Simple expression
5 + 3 * 2

// Evaluation steps:
// 1. Evaluate 3 * 2 → 6
// 2. Evaluate 5 + 6 → 11
```

### Evaluating Function Calls

```rust
fn square(x: i32) -> i32 {
    x * x
}

fn main() {
    let result = square(5);
}
```

**Conceptual evaluation:**

1. Call `square(5)`
2. Bind parameter: `x = 5`
3. Execute body: `5 * 5`
4. Return result: `25`
5. Bind to `result`: `result = 25`

### With Multiple Steps

```rust
fn f(x: i32) -> i32 {
    x * x - x - 1
}

const A: i32 = 13;

fn main() {
    let result = f(A - 1);
}
```

**Trace:**
```
f(A - 1)
→ f(13 - 1)        [substitute A]
→ f(12)            [evaluate subtraction]
→ 12 * 12 - 12 - 1 [substitute parameter]
→ 144 - 12 - 1     [evaluate multiplication]
→ 132 - 1          [evaluate first subtraction]
→ 131              [evaluate second subtraction]
```

---

## Error Handling in Rust

### Compile-Time Errors

Rust catches most errors **before** your code runs:

```rust
// Type mismatch
let x: i32 = "hello";  // ❌ ERROR: expected i32, found &str

// Undeclared variable
let y = z + 5;         // ❌ ERROR: cannot find value z

// Immutability violation
let x = 5;
x = 6;                 // ❌ ERROR: cannot assign twice to immutable variable
```

### Runtime Errors (Panics)

Some errors can only be detected at runtime:

```rust
// Division by zero
let x = 5 / 0;         // ⚠️ PANIC at runtime

// Integer overflow (in debug mode)
let x: u8 = 255;
let y = x + 1;         // ⚠️ PANIC in debug, wraps to 0 in release

// Array out of bounds
let arr = [1, 2, 3];
let x = arr[10];       // ⚠️ PANIC at runtime
```

### The Result Type (Proper Error Handling)

Rust encourages explicit error handling:

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

// Using the result
match divide(10, 2) {
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Error: {}", e),
}
```

---

## Type System Deep Dive

### Primitive Types

```rust
// Signed integers
i8, i16, i32, i64, i128, isize

// Unsigned integers  
u8, u16, u32, u64, u128, usize

// Floating point
f32, f64

// Boolean
bool

// Character (Unicode scalar)
char

// Unit type (like void)
()
```

### Type Inference vs Explicit Annotation

```rust
// Rust can often infer types
let x = 5;           // i32 inferred
let y = 3.14;        // f64 inferred

// But you can be explicit
let x: i32 = 5;
let y: f64 = 3.14;

// Sometimes you MUST be explicit
let x = "42".parse().expect("Not a number!");  // ❌ Ambiguous
let x: i32 = "42".parse().expect("Not a number!");  // ✅ Clear
```

### Type Conversions

```rust
// Implicit conversion does NOT exist
let x: i32 = 5;
let y: i64 = x;        // ❌ ERROR

// Must use explicit cast
let y: i64 = x as i64; // ✅ OK

// Or conversion methods
let y: i64 = i64::from(x);  // ✅ OK (infallible conversion)
```

---

## 📝 Quiz 4: Type System Deep Dive

### Multiple Choice

**Q1: Which of these is a valid type conversion?**
- A) `let x: i64 = 5_i32;`
- B) `let x: i64 = 5_i32 as i64;`
- C) `let x = i64::from(5_i32);`
- D) Both B and C

<details>
<summary>Answer</summary>

**D) Both B and C** are valid.
- A fails: no implicit conversion
- B works: explicit cast using `as`
- C works: using the `From` trait

Note: `i64::from()` is preferred when the conversion is infallible (can't fail), as it's more explicit about intent.
</details>

**Q2: What's the size of a `char` in Rust?**
- A) 1 byte
- B) 2 bytes
- C) 4 bytes
- D) Variable size

<details>
<summary>Answer</summary>

**C) 4 bytes** - A Rust `char` is a Unicode Scalar Value, which requires 4 bytes to represent any valid Unicode code point. This is different from C's `char` (1 byte) or Java's `char` (2 bytes).
</details>

### Code Analysis

**Q3: What's the output? If it errors, explain why.**
```rust
fn main() {
    let x = "42";
    let y = x.parse().unwrap();
    println!("{}", y + 8);
}
```

<details>
<summary>Answer</summary>

**Compilation error:** "type annotations needed"

The compiler can't infer what type to parse into. Fix:
```rust
let y: i32 = x.parse().unwrap();
// or
let y = x.parse::<i32>().unwrap();
```
</details>

### Written Response Questions

**Q4: Explain why Rust doesn't allow implicit type conversions between numeric types, even "safe" ones like i32 → i64.**

<details>
<summary>Answer</summary>

**Rust's philosophy: Explicitness over convenience**

**Reasons for no implicit conversions:**

1. **Clarity:** Makes all type changes visible in code
2. **Prevents bugs:** No unexpected precision loss or overflow
3. **Consistency:** Same rules apply everywhere
4. **Performance transparency:** Conversions aren't hidden

**Example of problems in C:**
```c
// C code - compiles with warnings
int large = 2147483647;
short small = large;  // Implicit truncation - data loss!
```

**Rust forces you to be explicit:**
```rust
let large: i32 = 2147483647;
// let small: i16 = large;           // ❌ Error
let small: i16 = large as i16;       // ✅ Explicit truncation
let small = i16::try_from(large);    // ✅ Returns Result
```

This explicitness makes bugs much more obvious.
</details>

**Q5: Consider this code. Why does it fail, and what are THREE different ways to fix it?**
```rust
fn average(a: i32, b: i32) -> f64 {
    (a + b) / 2
}
```

<details>
<summary>Answer</summary>

**Problem:** Integer division returns `i32`, but function signature promises `f64`.

**Fix #1: Convert before division**
```rust
fn average(a: i32, b: i32) -> f64 {
    (a as f64 + b as f64) / 2.0
}
```

**Fix #2: Convert after division (less accurate!)**
```rust
fn average(a: i32, b: i32) -> f64 {
    ((a + b) / 2) as f64
}
// Note: This truncates before converting, losing precision
```

**Fix #3: Convert sum, then divide**
```rust
fn average(a: i32, b: i32) -> f64 {
    (a + b) as f64 / 2.0
}
```

**Best practice:** Fix #1, as it maintains precision by working in floating point throughout.
</details>

---

## Compound Types

### Tuples (Fixed-Size Heterogeneous)

```rust
// Create a tuple
let point: (i32, i32) = (10, 20);
let mixed: (i32, f64, char) = (42, 3.14, 'x');

// Access elements
let x = point.0;     // 10
let y = point.1;     // 20

// Destructuring
let (x, y) = point;
println!("x: {}, y: {}", x, y);

// Tuples in functions
fn swap(pair: (i32, i32)) -> (i32, i32) {
    let (a, b) = pair;
    (b, a)  // Return swapped
}

let result = swap((1, 2));  // (2, 1)
```

### Arrays (Fixed-Size Homogeneous)

```rust
// Create an array
let numbers: [i32; 5] = [1, 2, 3, 4, 5];

// All elements same value
let zeros = [0; 100];  // 100 zeros

// Access elements (zero-indexed)
let first = numbers[0];   // 1
let third = numbers[2];   // 3

// Length known at compile time
const LEN: usize = 5;
let arr: [i32; LEN] = [1, 2, 3, 4, 5];

// Iterate
for element in &numbers {
    println!("{}", element);
}
```

**Key Difference from Racket Lists:**
- Arrays have **fixed size** known at compile time
- All elements must have the **same type**
- Direct indexing is O(1), not O(n)

---

## 🎯 Concept Check 4: Tuples and Arrays

### Quick Understanding

**Q1: What's the difference between `(1, 2, 3)` and `[1, 2, 3]`?**

<details>
<summary>Answer</summary>

**Tuple `(1, 2, 3)`:**
- Can hold different types: `(1, 3.14, 'x')` is valid
- Access by position: `.0`, `.1`, `.2`
- Size part of type: `(i32, i32, i32)` ≠ `(i32, i32)`
- Heterogeneous

**Array `[1, 2, 3]`:**
- All elements same type only
- Access by index: `[0]`, `[1]`, `[2]`
- Size part of type: `[i32; 3]` ≠ `[i32; 4]`
- Homogeneous
</details>

**Q2: What's the type of `let x = (5, "hello", 3.14);`?**

<details>
<summary>Answer</summary>

`(i32, &str, f64)` - a tuple containing an integer, a string slice, and a float.
</details>

### Code Challenges

**Q3: Debug this code:**
```rust
let tuple = (10, 20, 30);
let first = tuple[0];
println!("{}", first);
```

<details>
<summary>Answer</summary>

**Error:** Cannot use bracket notation on tuples.

**Fix:**
```rust
let tuple = (10, 20, 30);
let first = tuple.0;  // Use dot notation
println!("{}", first);
```
</details>

**Q4: What's wrong here?**
```rust
let numbers = [1, 2, 3, 4, 5];
numbers[2] = 10;
```

<details>
<summary>Answer</summary>

**Error:** Arrays are immutable by default.

**Fix:**
```rust
let mut numbers = [1, 2, 3, 4, 5];
numbers[2] = 10;  // ✅ Now it works
```
</details>

### Written Response Questions

**Q5: Write a function that takes a tuple of three integers and returns their sum.**

<details>
<summary>Answer</summary>

```rust
fn sum_tuple(nums: (i32, i32, i32)) -> i32 {
    nums.0 + nums.1 + nums.2
}

// Or with destructuring (cleaner):
fn sum_tuple(nums: (i32, i32, i32)) -> i32 {
    let (a, b, c) = nums;
    a + b + c
}

// Usage:
let result = sum_tuple((5, 10, 15));  // 30
```
</details>

**Q6: Explain when you'd use a tuple vs an array. Give specific examples.**

<details>
<summary>Answer</summary>

**Use tuples when:**
- You need different types together
- Fixed, small number of values with different meanings
- Returning multiple values from a function

**Examples:**
```rust
// Different types - must be tuple
let student: (&str, i32, f64) = ("Ruz", 19, 95.73);

// Function returning multiple values
fn divide_with_remainder(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}

// Coordinates (could be array, but tuple is clearer)
let point: (f64, f64) = (3.5, 7.2);
```

**Use arrays when:**
- All elements are the same type
- You need to iterate over elements
- Size is known at compile time
- Collection of similar items

**Examples:**
```rust
// Same type, need to iterate
let grades: [i32; 5] = [95, 87, 92, 88, 94];
let average: f64 = grades.iter().sum::<i32>() as f64 / grades.len() as f64;

// Matrix row
let row: [i32; 4] = [1, 0, 0, 1];

// Buffer of fixed size
let buffer: [u8; 1024] = [0; 1024];
```
</details>

---

## Common Patterns and Idioms

### Multiple Variable Bindings

```rust
// Declare multiple variables
let x = 5;
let y = 10;
let z = 15;

// Can also destructure
let (a, b, c) = (1, 2, 3);
```

### Temporary Calculations

```rust
fn hypotenuse(a: f64, b: f64) -> f64 {
    let a_squared = a * a;
    let b_squared = b * b;
    (a_squared + b_squared).sqrt()
}
```

### Early Returns

```rust
fn absolute_value(x: i32) -> i32 {
    if x < 0 {
        return -x;  // Early return
    }
    x  // Normal return
}
```

---

## Best Practices

### 1. Prefer Immutability

```rust
// ✅ Good - immutable by default
let x = calculate_value();

// ⚠️ Only when necessary
let mut counter = 0;
counter += 1;
```

### 2. Use Descriptive Names

```rust
// ❌ Bad
let x = 42;
let y = x * 2;

// ✅ Good
let student_count = 42;
let total_capacity = student_count * 2;
```

### 3. Explicit Types for Clarity

```rust
// OK but unclear
let result = calculate();

// Better
let result: f64 = calculate();

// Best - clear function signature
fn calculate() -> f64 {
    // ...
}
```

### 4. Keep Functions Small and Focused

```rust
// Each function does ONE thing well
fn parse_input(input: &str) -> i32 { /* ... */ }
fn validate_range(value: i32) -> bool { /* ... */ }
fn process_value(value: i32) -> String { /* ... */ }
```

---

## Compilation and Execution

### The Rust Compiler (rustc)

```bash
# Compile a Rust file
rustc main.rs

# Run the executable
./main
```

### Cargo (Rust's Build System)

```bash
# Create a new project
cargo new my_project

# Build the project
cargo build

# Build and run
cargo run

# Check for errors without building
cargo check

# Build optimized version
cargo build --release
```

### Project Structure

```
my_project/
├── Cargo.toml    # Project manifest
└── src/
    └── main.rs   # Source code
```

**Cargo.toml example:**
```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
# External crates go here
```

---

## Common Beginner Mistakes

### 1. Forgetting Mutability

```rust
let x = 5;
x += 1;  // ❌ ERROR

let mut x = 5;
x += 1;  // ✅ OK
```

### 2. Type Mismatches

```rust
let x = 5 + 2.5;  // ❌ ERROR

let x = 5.0 + 2.5;      // ✅ OK
let x = 5 as f64 + 2.5; // ✅ OK
```

### 3. Semicolon Confusion

```rust
fn broken() -> i32 {
    5 + 3;  // ❌ Returns (), not 8
}

fn working() -> i32 {
    5 + 3   // ✅ Returns 8
}
```

### 4. Borrowing Confusion (Preview)

```rust
let s = String::from("hello");
let s2 = s;         // Ownership moved
// println!("{}", s);  // ❌ ERROR: s no longer valid
println!("{}", s2);    // ✅ OK
```

---

## Tracing Practice Problems

### Problem 1

Assume these definitions:
```rust
const A: i32 = 13;

fn f(x: i32) -> i32 {
    x * x - x - 1
}
```

Trace: `f(A - 1)`

**Solution:**
```
f(A - 1)
→ f(13 - 1)       [substitute constant]
→ f(12)           [evaluate subtraction]
→ 12*12 - 12 - 1  [substitute parameter]
→ 144 - 12 - 1    [evaluate multiplication]
→ 132 - 1         [evaluate first subtraction]  
→ 131             [final result]
```

---

### Problem 2

Assume these definitions:
```rust
const A: i32 = 12;
const B: i32 = A + 11;

fn f(x: i32, y: i32) -> i32 {
    x + A + y + B
}
```

Trace: `f(A + B, 1)`

**Solution:**
```
f(A + B, 1)
→ f(12 + 23, 1)          [substitute A and B]
→ f(35, 1)               [evaluate addition]
→ 35 + A + 1 + B         [substitute parameters]
→ 35 + 12 + 1 + 23       [substitute A and B]
→ 47 + 1 + 23            [left to right]
→ 48 + 23                [continue]
→ 71                     [final result]
```

---

### Problem 3 (Error Tracing)

```rust
const A: i32 = 7;

fn quux(x: i32) -> i32 {
    x * 2
}

fn main() {
    let result = A + quux(A) / 0;
}
```

**Trace:**
```
A + quux(A) / 0
→ 7 + quux(7) / 0        [substitute A]
→ 7 + (7 * 2) / 0        [evaluate quux(7)]
→ 7 + 14 / 0             [evaluate multiplication]
→ 7 + [PANIC]            [division by zero!]
ERROR: thread 'main' panicked at 'attempt to divide by zero'
```

---

## Key Takeaways

1. **Rust is explicit** - types, mutability, errors must be handled
2. **The compiler is your friend** - strict checking prevents bugs
3. **Ownership matters** - every value has exactly one owner
4. **Immutable by default** - must opt-in to mutability  
5. **No implicit conversions** - be explicit about types
6. **Expressions over statements** - last expression is returned
7. **Safety without garbage collection** - zero-cost abstractions

---

## Comparison: Racket vs Rust

| Feature | Racket | Rust |
|---------|--------|------|
| **Paradigm** | Functional | Multi-paradigm |
| **Typing** | Dynamic | Static, strong |
| **Memory** | Garbage collected | Ownership system |
| **Syntax** | Prefix, minimal | Infix, verbose |
| **Mutability** | Functional style | Explicit `mut` |
| **Performance** | Interpreted | Compiled, fast |
| **Use Case** | Teaching, DSLs | Systems, performance |

---

## Next Steps

After mastering these basics, you'll learn about:

- **Ownership and Borrowing** (Rust's superpower)
- **Structs and Enums** (custom types)
- **Pattern Matching** (powerful control flow)
- **Error Handling** (Result and Option)
- **Collections** (Vec, HashMap, etc.)
- **Lifetimes** (advanced ownership)
- **Traits** (interfaces and polymorphism)
- **Concurrency** (fearless parallel programming)

---

## Resources

- **The Rust Book**: https://doc.rust-lang.org/book/
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
- **Rustlings**: Small exercises to get you started
- **Rust Playground**: https://play.rust-lang.org/

---

## 📝 Final Comprehensive Quiz

### Section A: Trace Execution (Show Your Work)

**Q1: Trace through this code step by step:**
```rust
const X: i32 = 5;

fn double(n: i32) -> i32 {
    n * 2
}

fn add_three(n: i32) -> i32 {
    n + X - 2
}

fn main() {
    let result = add_three(double(X + 1));
}
```

<details>
<summary>Answer</summary>

```
add_three(double(X + 1))
→ add_three(double(5 + 1))          [substitute X]
→ add_three(double(6))              [evaluate addition]
→ add_three(6 * 2)                  [substitute parameter in double]
→ add_three(12)                     [evaluate multiplication]
→ 12 + X - 2                        [substitute parameter in add_three]
→ 12 + 5 - 2                        [substitute X]
→ 17 - 2                            [left to right]
→ 15                                [final result]
```
</details>

**Q2: What does this print? Trace the shadowing:**
```rust
fn main() {
    let x = 10;
    let x = x * 2;
    
    {
        let x = x + 5;
        println!("Inner: {}", x);
    }
    
    let x = x - 3;
    println!("Outer: {}", x);
}
```

<details>
<summary>Answer</summary>

**Trace:**
1. First binding: `x = 10`
2. Shadow: `x = 10 * 2 = 20`
3. Enter block, shadow: `x = 20 + 5 = 25`
4. Print "Inner: 25"
5. Exit block (block's x destroyed, back to outer x = 20)
6. Shadow: `x = 20 - 3 = 17`
7. Print "Outer: 17"

**Output:**
```
Inner: 25
Outer: 17
```
</details>

### Section B: Debug & Fix

**Q3: This function should calculate the area of a triangle (½ × base × height). Find and fix all errors:**
```rust
fn triangle_area(base, height) {
    base * height / 2
}
```

<details>
<summary>Answer</summary>

**Errors:**
1. Missing parameter types
2. Missing return type
3. Integer division will truncate

**Fixed:**
```rust
fn triangle_area(base: f64, height: f64) -> f64 {
    base * height / 2.0
}

// Or if you want integer inputs but float output:
fn triangle_area(base: i32, height: i32) -> f64 {
    (base * height) as f64 / 2.0
}
```
</details>

**Q4: Find and fix the scope error:**
```rust
fn calculate() -> i32 {
    let result = 42;
}

fn main() {
    println!("{}", result);
}
```

<details>
<summary>Answer</summary>

**Errors:**
1. `result` doesn't exist in `main` scope
2. `calculate()` doesn't return anything (missing return)

**Fix:**
```rust
fn calculate() -> i32 {
    let result = 42;
    result  // or: return result;
}

fn main() {
    let result = calculate();
    println!("{}", result);
}
```
</details>

### Section C: Type System Mastery

**Q5: Explain what's wrong and provide TWO different fixes:**
```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let sum = 0;
    
    for n in &numbers {
        sum = sum + n;
    }
    
    println!("Sum: {}", sum);
}
```

<details>
<summary>Answer</summary>

**Error:** `sum` is immutable, can't reassign in loop.

**Fix #1: Make it mutable**
```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let mut sum = 0;  // Add mut
    
    for n in &numbers {
        sum = sum + n;
    }
    
    println!("Sum: {}", sum);
}
```

**Fix #2: Use functional approach (more idiomatic)**
```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    
    println!("Sum: {}", sum);
}
```
</details>

**Q6: Type inference challenge - what are the inferred types?**
```rust
let a = 42;
let b = 3.14;
let c = (a, b);
let d = [1, 2, 3];
let e = "hello";
let f = 'x';
```

<details>
<summary>Answer</summary>

```rust
let a = 42;           // i32
let b = 3.14;         // f64
let c = (a, b);       // (i32, f64)
let d = [1, 2, 3];    // [i32; 3]
let e = "hello";      // &str
let f = 'x';          // char
```
</details>

### Section D: Design & Implementation

**Q7: Write a function `celsius_to_fahrenheit` that converts temperature. Formula: F = C × 9/5 + 32**

<details>
<summary>Answer</summary>

```rust
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

// Test it:
fn main() {
    println!("0°C = {}°F", celsius_to_fahrenheit(0.0));    // 32
    println!("100°C = {}°F", celsius_to_fahrenheit(100.0)); // 212
    println!("-40°C = {}°F", celsius_to_fahrenheit(-40.0)); // -40
}
```

**Common mistake to avoid:**
```rust
// ❌ Don't do this (integer division!)
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9 / 5 + 32  // 9/5 = 1 in integer math!
}
```
</details>

**Q8: Write a function that takes three test scores and returns a tuple of (average, highest, lowest).**

<details>
<summary>Answer</summary>

```rust
fn analyze_scores(s1: f64, s2: f64, s3: f64) -> (f64, f64, f64) {
    let average = (s1 + s2 + s3) / 3.0;
    
    let highest = if s1 >= s2 && s1 >= s3 {
        s1
    } else if s2 >= s3 {
        s2
    } else {
        s3
    };
    
    let lowest = if s1 <= s2 && s1 <= s3 {
        s1
    } else if s2 <= s3 {
        s2
    } else {
        s3
    };
    
    (average, highest, lowest)
}

// Better with built-in methods:
fn analyze_scores(s1: f64, s2: f64, s3: f64) -> (f64, f64, f64) {
    let average = (s1 + s2 + s3) / 3.0;
    let highest = s1.max(s2).max(s3);
    let lowest = s1.min(s2).min(s3);
    (average, highest, lowest)
}

// Usage:
let (avg, high, low) = analyze_scores(85.0, 92.0, 78.0);
```
</details>

### Section E: Conceptual Understanding

**Q9: Essay Question: Explain Rust's "zero-cost abstractions" principle. How does it relate to the type system and compile-time checking?**

<details>
<summary>Answer</summary>

**Zero-Cost Abstractions:**

Rust provides high-level abstractions (safety, ownership, traits, generics) without runtime overhead. You don't pay for features you don't use, and features you do use compile to code as fast as if you wrote it by hand.

**Relationship to Type System:**

1. **Compile-time checking:** All type checking happens during compilation. By runtime, types are resolved—no need for runtime type tags or dynamic dispatch (unless explicitly requested via trait objects).

2. **Memory layout:** The compiler knows exact sizes and layouts at compile time, enabling optimal memory usage without runtime overhead.

3. **Optimization:** Strong type information enables aggressive compiler optimizations. For example:
```rust
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}
```
This compiles to a single CPU instruction—no function call overhead with optimization.

4. **Safety without runtime cost:** Borrow checker runs at compile time. There's no garbage collector running at runtime, yet memory safety is guaranteed.

**Example:**
```rust
// High-level abstraction
let numbers = vec![1, 2, 3, 4, 5];
let sum: i32 = numbers.iter().sum();

// Compiles to code as efficient as:
let mut sum = 0;
for i in 0..5 {
    sum += numbers[i];
}
```

The abstraction (iterator, sum method) costs nothing at runtime.
</details>

**Q10: Why does Rust require explicit type annotations for function parameters but not always for return types or local variables? What principle guides this design?**

<details>
<summary>Answer</summary>

**Design Principle: Explicit at boundaries, inference within**

**Function parameters MUST be annotated because:**
1. **Interface clarity:** Functions are API boundaries. Explicit types serve as documentation.
2. **Separate compilation:** The compiler needs to know parameter types without examining function bodies.
3. **Error locality:** Type errors are reported at the call site, not deep in the implementation.
4. **Prevents cascading ambiguity:** Without parameter types, the compiler would need to infer them from usage, leading to confusing errors.

**Return types CAN be inferred because:**
- They can be determined from the function body
- But explicit return types are strongly encouraged for public APIs

**Local variables CAN be inferred because:**
- They're internal to a function
- The compiler has full context
- Inference reduces boilerplate

**Example:**
```rust
// ❌ This fails - can't infer parameter type
fn double(x) -> i32 { x * 2 }

// ✅ This works - can infer return type
fn double(x: i32) { x * 2 }  // Returns i32 (inferred)

// ✅ Best - explicit for clarity
fn double(x: i32) -> i32 { x * 2 }

// Inside function - inference works great
fn calculate(x: i32) -> i32 {
    let doubled = x * 2;      // i32 inferred
    let tripled = x * 3;      // i32 inferred
    doubled + tripled         // i32 inferred
}
```

This balance provides safety (at API boundaries) with convenience (internally).
</details>

---

## Practice Exercises

### Exercise 1: Temperature Conversion
Write a function that converts Fahrenheit to Celsius using the formula: C = (F - 32) × 5/9

```rust
fn fahrenheit_to_celsius(f: f64) -> f64 {
    // Your code here
}
```

### Exercise 2: Circle Calculations
Write functions to calculate:
- Area of a circle given radius
- Circumference of a circle given radius

```rust
fn circle_area(radius: f64) -> f64 {
    // Your code here
}

fn circle_circumference(radius: f64) -> f64 {
    // Your code here
}
```

### Exercise 3: Quadratic Formula
Implement the quadratic formula to find one root: x = (-b + √(b² - 4ac)) / 2a

```rust
fn quadratic_root(a: f64, b: f64, c: f64) -> f64 {
    // Your code here
}
```

---

**End of Module 01: Introduction to Rust**

*Remember: The Rust compiler is strict because it cares about your program's safety. Embrace the errors - they're learning opportunities!*