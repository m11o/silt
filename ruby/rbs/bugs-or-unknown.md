# [RBS] キーワード引数がrequiredではない場合に、エラーの文言が不明瞭な部分がある

requiredではないキーワード引数の書き方は以下の通りである

```
def foo(String a, ?b: Integer) -> void
```

ただし、 `?` をbの後ろにつけてしまった場合には、steepでは以下のようなエラーが発生する

```
More keyword arguments are required: b?
```

でもこれって、そもそも `b?` はruby的にはsyntaxエラーなのでエラー文言がおかしい気がするー
そもそもエラーになって欲しい
