#[macro_export]

macro_rules! sum_range{
    ()=> {0};

    ($($x:expr),+ $(,)?) => {
        0 $(+ $x)*
    }
}