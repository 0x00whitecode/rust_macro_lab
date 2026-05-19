

#[macro_export]
macro_rules! my_vec {
    ( $( $x:expr ),* $(,)? ) => {
        {
            let mut vec = Vec::new();
            $(
                vec.push($elem);


            )*
            vec
        }
    };

    () => {
        Vec::new()
    };
}