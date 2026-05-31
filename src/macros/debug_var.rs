#[export_macro]


macro_rules! debug_var {
    ($var: expr) => {
        println!("{} = {:?}", stringify!($var), $var);
    }
}