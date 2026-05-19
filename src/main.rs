mod macros;



fn main() {
    // task 0ne
    hello_world!();
    // task two
    println!("{}", calculate!(2 + 3));
    println!("{}", calculate!(2 * 3));
    println!("{}", calculate!(2 / 3));
    // println!("{}", calculate!()); for the panic
    // task three
    println!("task three");
     println!("{}", sum_range!()); // 0
    println!("{}", sum_range!(1)); // 1
    println!("{}", sum_range!(1, 2, 3)); // 6
    println!("{}", sum_range!(1, 2, 3,)); // 6

    // task four
    log_info!("This is an info message.");
}
