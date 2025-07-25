fn question4() {
    println!("Generate the nth Fibonacci number.");
    let n = 4;
    let y = fibonacci(n);
    println!("fib({n}) = {y}");
}


fn fibonacci(n: usize) -> u64 {
    let (mut a, mut b) = (0, 1);
    let mut i = 0;

    while i < n {
        let next = a + b;
        a = b;
        b = next;
        i += 1;
    }

    a
}
