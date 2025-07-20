#[derive(Debug)]
struct PasswordChecker {
    score: i32,
    feedback: String,
}

fn question1() {
    println!("Exercise for Lesson 1 till Lesson 5");
    println!("Question 1: Create a function that takes a password string and returns a strength score \nRules: +1 for length > 8, +1 for numbers, +1 for special chars \nUse structs to represent the result");
    let x = password_checker("eeeeeeee^%%$%^$%46756756675e");
    println!("{x}");
}

// Create a function that takes a password string and returns a strength score
// Rules: +1 for length > 8, +1 for numbers, +1 for special chars
// Use structs to represent the result

fn password_checker(password: &str) -> i32 {
    let mut score: i32 = 0;
    
    // Check length
    if password.len() > 8 {
        score += 1;
    }
    
    // Check for numbers (using flags)
    let mut has_numbers = false;
    let mut has_special = false;
    
    for char in password.chars() {
        if char.is_numeric() {
            has_numbers = true;  // Just mark that we found one
        } else if !char.is_alphabetic() {
            has_special = true;  // Found a special character
        }
    }
    
    // Add points based on what we found
    if has_numbers {
        score += 1;
    }
    if has_special {
        score += 1;
    }
    
    score  
}