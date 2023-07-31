# implにlifetimeをつける方法

```rust
impl <'a> Request {
    fn hoge(&'a self) -> &'a str {
        "hoge"
    }
}
```

ライフサイクルについてはもっと調べる必要がありそう
