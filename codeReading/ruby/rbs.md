# rbs

github: https://github.com/ruby/rbs

# assert_send_type

ref: https://github.com/ruby/rbs/blob/f44f5d398a54889c74ef8323c2fc518003012529/lib/rbs/unit_test/type_assertions.rb#L171

## send_setup

メソッドを実行してみて、その結果を取得している気がする
Spyが何をしているのか

## Spy

無名クラスを定義して、渡されたオブジェクトをreceiverにその中でメソッドを実行する。
そのメソッドの返り値はblockからの返り値を以下のフォーマットで保存しておく。

```ruby
{
  arguments: [], # メソッドに渡される引数
  value: nil,    # メソッドの返り値を入る
}
```
