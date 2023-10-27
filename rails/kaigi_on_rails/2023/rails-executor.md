# rails executor

evilmartians
=> martian open source

martian open source project

## 問題点と使い方

Railsが遅いわけではない -> railsが色々やってくれているから

書いているコードよりも、動いているコードが多い

resource management

railsで動くgemを作りたいなら、railsを知る必要がある

rails executor
=> framework コードとapplicationコードの境界線

action, jobの全てで使用されている

to_run, to_complete
コードの再読み込みを行わない

rails reloaderはアプリケーションのコードを実行する前に、最新のコードを読み込む

```ruby
Rails.applivation.executor.wrap do
  # do something
end
```

```ruby
Rails.application.executor.to_run do
  
end
```

rails worldでアーキテクチャを説明したslideがある
-> これは読む価値がありそう

github: envek
github: evilmartian
