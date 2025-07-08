# YSTD
An extension for many standard library types to provide a better default experience, esspecially in the world of async, namely:
- Assumes UTF8 everywhere, especially paths
- All IO is asynchronous by default, using `tokio`
- All error handling is done in conjunction with `color_eyre::Report` for beautiful error messages by default
- `Option<T>`'s are frequently replaced with `colour_eyre::Result<T>` for better error messages and `?` instead of `.unwrap()` in faster development scenarios
- The most restrictive, safest option is made readily available (mostly because it is implemented first) in cases where it is appropriate,
to speed up thinking time deciding how open to allow your programs to be, e.g. `ystd::base64::decode` uses the `::base64::BASE64_STANDARD` engine only
