fn main() {
    println!("Hello World");

    //SIMPLE IF 
    let number = 6;
    if number < 5 {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }
    
    //if in an let statement
    let condition = true;

    let num = if condition {
        5
    } else {
        6
    };
    println!("The number is {num}");

    /*
    && = AND
    || = OR
     */

    //match case
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    //define coin
    let coin = Coin::Penny;

    //print value of coni
    println!("Valye of coin is: {}", value_in_cents(coin));

    //loops
    // loop {
    //     println!("I AM A LOOOOOOPPPPPPPPP");
    // }

    let mut i = 0;
    let result = loop {
        i += 1;

        if i == 10 {
            break i
        }
    };
    println!("{i}");

    //while loop
    //same logic as the normal loop

    //for loop
    let a = [1, 2, 3, 4, 5, 6, 7, 8];

    for element in a.iter() {
        println!("The value is: {}", element);
    }

    let s = "hello world";

    for c in s.chars() {
        println!("The value is {c}");
    }
}

