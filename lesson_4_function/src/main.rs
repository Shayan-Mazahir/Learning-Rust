//function is set of statements that  perform a task
fn main() {
    let num1: i32 = 23;
    let num2: i32 = 20;

    println!("Hello, world!");
    simple_function();
    two_parameter(num1, num2);
    another_function();
    another_another_function();
    let result = return_function(10, 20);
    println!("x: {}", result.0);
    println!("y: {}", result.1);
}

//a simple function
fn simple_function() {
    println!("Me is a simple function")
}

//a function with 2 parameter
fn two_parameter(num1: i32, num2: i32) {
    println!("I have 2 parameter");
    println!("The two numbers are: {num1} and {num2}. There sum are: {}", num1 + num2);
}

//Statements and Expressions
//instructions that perform action but on return value hence no ;
//expression evaluate and return a value

fn another_function() {

    let x: i32 = 5;
    let y: i32 = 6;
    let z: i32 = x + y;
    println!("The value of z is: {z}");
}

fn another_another_function() {
    let y: i32 = {
        let x: i32 = 3;
        x + 2
    };
    println!("The value of y is: {y}")
}

//return values
fn return_function(num1: i32, num2: i32) -> (i32, String) {
    let x = num1 + num2;
    let y = format!("The sum of the two numbers is: {}", x);
    (x, y)
}
