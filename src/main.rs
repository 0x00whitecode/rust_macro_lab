mod macros;



fn main() {
    hello_world!();
    println!("{}", calculate!(2 + 3));
    println!("{}", calculate!(2 * 3));
    println!("{}", calculate!(2 / 3));
    println!("{}", calculate!());
}
