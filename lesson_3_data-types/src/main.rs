//STRUCTS
#[derive(Debug)] //why do we need this though?
struct Person {
    name: String,
    age: u8,
}

fn main() {
    println!("There are different data types in rust. Scalar, Compund and Custom types.");
    println!("Scalar Types: \n 1. Integers:");

    /*
    Length  Signed (can store -ve and +ve)  Unsigned (only +ve)
    8-bit   i8                               u8
    16-bit  i16                              u16
    32-bit  i32                              u32
    64-bit  i64                              u64
    128-bit i128                             u128
    arch    isize                            usize
    */

    let decimal = 9820;
    let hex = 0xff; //converts
    let octal = 0o77; //converts
    let binary = 01010101;
    let byte = b'A'; //converts

    println!("Decimal is {}", decimal);
    println!("Hex is {}", hex);
    println!("Octal is {}", octal);
    println!("Binary is {}", binary);
    println!("Byte is {}", byte);

    //FLOATING POINTS
    let x: f32 = 2.5;
    let y = 2.0;

    println!("Addition of x and y gives us {}", x+y);
    println!("Division of x and y gives us {}", x/y);
    println!("Remainder of x and y gives us {}", x%y);

    //Tuples
    let tup: (i32, f64, char) = (500, 6.4, 'x');

    let (x, y, z) = tup;
    println!("The value of y is {}", y);

    println!("The value of 500 is: {}", tup.0);
    //you can access a tuple like a list/dictionary


    //ARRAYS
    let arr = [1, 2, 3, 4, 5]; //array of 5 elements
    
    println!("First value in the array is {}", arr[0]);


    //custom type: stricts
    let person = Person {
        name: String::from("John"),
        age: 25,
    };
    print!("Person is {:?}", person);

}