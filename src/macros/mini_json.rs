use std::collections::HashMap;

#[macro_export]

macro_rules! mini_json {

    // empty case
    ({}) => {
        {
            HashMap::new()
        }
    };

    ({ $key:tt : $value:tt $(, $rest:tt)* }) => {
        {
            let mut map = mini_json!({ $($rest),* });
            map.insert(String::from(stringify!($key)), String::from(stringify!($value)));
            map
        }
    };

}
