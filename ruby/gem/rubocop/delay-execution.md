# Rubocopの処理が遅い

Rails serverを立てた状態で、rubocopを実行すると明らかに遅くなっている
どこかに律速になっている箇所があるはず
要調査

再現方法

```shell
bundle exec rails s
bundle exec rubocop --debug
```