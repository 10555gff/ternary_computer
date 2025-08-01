# Ternary Arithmetic

A Rust library use array logic gate to arithmetic.

a numeral system with digits `-1`, `0`, and `+1`.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
text_to_emoji = "0.1.1"
```

### Example

```rust
use text_to_emoji::convert_to_emojis;

let result = convert_to_emojis("I love coffee and pizza");
assert_eq!(result, "I ❤️ ☕️ and 🍕");
```

## License

This project is licensed under the MIT License.
