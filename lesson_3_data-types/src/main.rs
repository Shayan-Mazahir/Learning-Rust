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

    let number: i8 = -1;
    let number2: i8 = 1;
    
    println!("Number is {}", number);
    println!("Number is {}", number2);
    
    let number3: u8 = 1;

    println!("Number is {}", number3);

}
