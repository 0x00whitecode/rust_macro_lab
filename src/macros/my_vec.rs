

#[macro_export]
macro_rules! my_vec {
    ( $( $elem:expr ),* $(,)? ) => {
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