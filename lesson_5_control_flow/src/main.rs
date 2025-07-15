fn main() {
    println!("If Expressions");
    let number = 3;
    if number < 5 {
        println!("Number is less than 5");
    } else if number > 5 {
        println!("Number is grator than 5");
    } else if number == 5 {
        print!("Number is 5");
    }
    //works... yipieeee

    println!("If in a let statement");
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of the number is {number}");
    println!("Match constructor");
//match constructor
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

    println!("Loops");
    println!("The loop function (loop{{}})is commented out");
    // loop {
    //     println!("LOOOOOOPPPPP");
    // }
    println!("While loops");
    let mut i = 10;
    while i != 0 {
        println!("{i}");
        i -= 1;
    }
    println!("DOINE!");

    println!("Looping through an array");

    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut x: usize = 0;
    while x <= 9 {
        println!("The value at this index in the array is: {}", array[x]);
        x += 1;
    }

    for element in array {
        println!("Using the element instead of a variable to loop");
        println!("The value at this index in the array is: {element}")
    }

    //cooooolllll
    for number in (1..4).rev() {
        println!("{number}");
    }
    
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter
        }
    };

    println!("The result is {result}");
}

