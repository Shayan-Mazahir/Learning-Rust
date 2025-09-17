fn question5() {
    // println!("Hello from question 5");
    println!("Write a program which does the FizzBuzz");
    
    fizz_buzz();

    /*
    The basic challenge is to write a program that prints numbers from 1 to 100, but with a twist:
        If the number is divisible by 3, print "Fizz" instead
        If it's divisible by 5, print "Buzz" instead
        If it's divisible by both 3 and 5, print "FizzBuzz"
        Otherwise, just print the number
     */
}

fn fizz_buzz() {
    let mut i = 0;
    // let mut check_number: i32;

    while i <= 99 { //remember counts starts from 0 not 1
        if (i%3) == 0 && (i%5) == 0 {
            println!("FizzBuzz");
        } else if (i%3) == 0 {
            println!("Fizz");
        } else if (i%5) == 0 {
            println!("Buzz");
        } else {
             println!("{i}"); //in rust all the branches have to be the same return, meaning one cant be a string and other is an integer
        }
        
        i += 1;
    }
}


