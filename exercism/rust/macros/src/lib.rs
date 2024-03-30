#[macro_export]
macro_rules! hashmap {
    () => {
        ::std::collections::HashMap::new()
    };
    (
        $($key:expr => $value:expr),+
        // optional trailing comma
        $(,)?
    ) => {
        ::std::collections::HashMap::from(
            [
                $(($key, $value),)*
            ]
        )
    };
}
