# Option<T>がNoneの場合に、default値を返す

`unwrap_or`を使用する

```rust
let meaning = Parser::parse_meaning(responded_message.clone()).unwrap_or(String::from(""))
```
