#[macro_export]
macro_rules! hashmap {
    () => {
        ::std::collections::HashMap::new()
    };
    (
        $first_key:expr => $first_value:expr
        $(, $key:expr => $value:expr )*
        $(,)? // trailing comma allowed
    ) => {
        ::std::collections::HashMap::from(
            [
                ($first_key, $first_value)
                $(, ($key, $value))*
            ]
        )
    };
}
