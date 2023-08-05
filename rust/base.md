# implにlifetimeをつける方法

```rust
impl <'a> Request {
    fn hoge(&'a self) -> &'a str {
        "hoge"
    }
}
```

ライフサイクルについてはもっと調べる必要がありそう

# build.rs

build.rsはビルド時に実行されるスクリプトで、Cargo.tomlに以下のように記述することで、ビルド時に実行される
ルートディレクトリに追加する

```toml
[package]
...
build = "build.rs"
```

# traitについて

共通の振る舞いを定義することができる

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```
