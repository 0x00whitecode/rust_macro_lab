#[macro_export]

macro_rules! mini_html{

    ($($tag:ident => $content:expr), * $(,)?) => {{
        let mut html = String::new();
        $(html.push_str(
            &format!("<{}>{}</{}>", 
            stringify!($tag), $content,

            stringify!($tag)
        ));
        )*
        html
    }}
}