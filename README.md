# option-inspect-none

The crate contains a trait, that implements the inspect_none(..) method on Options.

## Usage example

```rust
let mut inspector_function_called = false;
None::<()>.inspect_none(|| inspector_function_called = true);
assert!(inspector_function_called);
```
