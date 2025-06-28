# YSTD
A replacement for many standard library types to provide a better default experience, namely:
- Assumes UTF8 everywhere, esspecially paths
- All IO is asynchronous by default, using `tokio`
- All error handling is done in conjunction with `color_eyre::Report` for beautiful error messages by default
- `Option<T>`'s are frequently replaced with `colour_eyre::Result<T>` for better error messages and `?` instead of `.unwrap()`
