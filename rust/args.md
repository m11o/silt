# コマンドラインオプション解析

`std::env` モジュールの `args` 関数を使用することによって、コマンドライン引数を取得することができます。

```rust
use std::env;

fn main() {
  let program: String = env::args().next().unwrap();
  let args: Vec<String> = env::args().skip(1).collect();
}
```

# 引数がない場合にエラーを返す方法

`std::env::args` 関数は、引数を受け取っていない場合、Noneが返るのでそれをパターンマッチで受け取ってpanic!させる

```rust
use std::env;

fn main() {
  match env::args().nth(1) {
    Some(arg) => println!("{}", arg),
    None => panic!("引数がありません"),
  }
}
```

## references

https://qiita.com/tatsuya6502/items/cd41599291e2e5f38a4a

# .envファイルから環境変数を読み込む

## dotenvクレートを使用する

github: https://github.com/dotenv-rs/dotenv

使い方

1. dotenv.ok()を使用して、std::env::varから値を取得する

```text:.env
DATABASE_URL=postgres://root:root@localhost/rust-dev
```

```rust
use std::env;
use dotenv::dotenv;

fn main() {
  dotenv().ok();
  
  let database_url = env::var("DATABASE_URL").unwrap();
}
```

2. keyを指定して値を取得する

```rust
use dotenv;

fn main() {
  let key = "FOO"
  let value = dotenv::var(key).unwrap();
}
```
