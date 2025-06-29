# Important commands to know

`cargo new (project_name)` -> inititalizing a new project
`cargo build` -> builds the whole project/compiles the whole project (?)
`cargo check` -> just checks if the code can compile without compiling
`cargo run` -> runs the program

`touch <file name>.rs` -> makes a new rs file
`rustc` -> compiles the .rs file

# Basics
## Varibles and Constants

1. Variable Mutability: Rust by default has a varible securing feature, meaning by default if we use `let` our variable is immutable (can not be changed; only the value can be changed, not the type). In order for us to change the variable type we should define our variable using `let mut <variable name>` this allows us to change the data type of the variable.

2. Variable Shadowing: Each `let` makes a new variable. This means, we can create as many same worded vraiable and have them given different data types. e.g:

```rust
let password = 123;
let password = "password123";
let password = 987;
//all these are new variables, different data types but same names; and ofc they are immutable
```

3. Scope: 
```rust
let user_role = "USER";           // Outer scope
{
    let user_role = "ADMIN";      // Inner scope (shadows outer)
    let temp_var = "SECRET";      // Only exists in this block
}
// user_role is back to "USER"
// temp_var doesn't exist here!
```
- Inner scopes can shadow outer variables
- When inner scope ends, shadowing disappears
- Variables only live within their scope

4. Constants: Same as any other language:

```rust
const MAX_LOGIN_ATTEMPTS: u32 = 3;  // Must specify type
const ADMIN_PRIVILEGES: &str = "ADMIN";
```

- Immutable forever: Cannot be changed, ever
- Compile-time values: Must be calculable at compile time
- Global scope: Available throughout the program
- Type required: Must explicitly specify the type 
- Naming convention: SCREAMING_SNAKE_CASE