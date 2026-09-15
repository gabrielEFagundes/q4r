#[macro_export]
macro_rules! emit {
    ($buf:expr, $($item:expr),+$(,)?) => {
        {
            $( AppendTo::append($item, $buf); )+
        }
    };
}