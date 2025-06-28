fn main() {
    let mut x:i32 = 5;
    //i32 is the data type... interesting...

    println!("The value of x is: {}", x);
    // the {} is a form of pretty formatting for rust, just like in python

    //in rust by default the varibale if used let is immutable, to make it mutable w euse mut
    x = 6;
    println!("The value of x is: {}", x);

    //shadowing
    let y = 5;
    println!("The value of x is {}", y);

    {
        let y = 8;
        println!("The value of y is {}", y);
    }

    let y = y + 5;
    println!("The value of y is {}", y);

    let y = "hi";
    println!("The value of y is: {}", y);
    //if we have a mut variable we can not change the type of that variable, we can only change the value; so:

    let mut z = 5;
    println!("Z = {}", z);

    /*IMPORTANT */

    //this wont print anyhting cause the z is a mutable variable, rust expects that z remains the same data type, but different value
    // z = "hola";
    // println!("Z = {}", z); //error

    //BUT:
    //this will work, because we are re-declearing the varibale as a mutable, hence changing the expected data type
    let mut z = "hola";
    println!("Z = {}", z);

    //constant
    const MAX_POINTS:i32 = 100_00;
    println!("The value of MAX_POINTS is {}", MAX_POINTS);

    //shadowing with constants? (well ofc no, what do u expect)
    // const MAX_POINTS:i32 = 100_00;
    // println!("The value of MAX_POINTS is {}", MAX_POINTS)
}
