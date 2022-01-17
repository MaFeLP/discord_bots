/// This macro has just been copied from the [once_cell documentation](https://docs.rs/once_cell/1.9.0/once_cell/index.html#lazily-compiled-regex)
#[macro_export]
macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}
