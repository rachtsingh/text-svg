![example](https://raw.githubusercontent.com/matthunz/text-svg/main/image.svg)

Text -> SVG path in rust

[Examples](https://github.com/matthunz/text-svg/tree/main/examples)

```rust
Text::builder()
    .size(50.0)
    .start(Point { x, y })
    .build(&font, "text-svg");
```
