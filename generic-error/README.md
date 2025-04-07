## Generic Error
A simple utility to convert any error type that isn't clonable into one that is,
while preserving source information providing `Serialize` and `Deserialize` functionality.
Magic?

## How it works
The `GenericError<E>` wrapper type just stores the `Display` and `Debug` implementations of the wrapped type,
which is now always clonable and will serialize just fine.
The `GenericError<E>` type kinda looks like this:
```rust
#[derive(Serialize, Deserialize, Clone)]
pub struct GenericError<T>
{
  display: String,
  debug: String,
  source: Option<GenericError>,
}
```
.. although many details have been omitted for brevity


See [docs](https://docs.rs/generic-err) or the
[examples](https://github.com/ActuallyHappening/YMap/tree/master/generic-error/examples) dir for usage
