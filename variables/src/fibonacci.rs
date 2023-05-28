pub fn fibonacci(loops:  u32) {
    let mut fib_first = 0;
    let mut fib_second = 1;

    println!("Welcome to the fibonacci generator !  Hop on board for {} loops!", loops);

    for loop_num in 0..loops {
        let fib_sum = fib_first + fib_second;

        //println!("loop#{}, first: {}, second: {}, sum: {}", loop_num, fib_first, fib_second, fib_sum);
        println!("{}:, {}", loop_num, fib_first);

        fib_first = fib_second;
        fib_second = fib_sum;
    }
}