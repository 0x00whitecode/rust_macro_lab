#[macro_export]
macro_rules! calculate {

    ($expr:expr) => {
        $expr
    };

    () => {
        panic!("Usage: calculate!(a + b), calculate!(a - b), etc.")
    };
}