fn question3() {
    println!("Convert temperatures between Fahrenheit and Celsius.");
    let temp: f64 = 104.9;
    let temp_c: f64 = converstion(temp);
    println!("{temp_c}");

}

//function to convert from fahrenheit to celsius
fn converstion(temp: f64) -> f64 {
    let final_temp: f64 = (temp - 32.0) * (5.0/9.0);
    final_temp
}