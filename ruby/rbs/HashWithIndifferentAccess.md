# HashWithIndifferentAccess

HashWithIndifferentAccessは、keyがstringでsymbolでもどちらもアクセスできるよっていうHashクラスのオブジェクトだから、型は

```ruby
HashWithIndifferentAccess[T, U] < Hash[T, U]
```

ではなく

```ruby.rbs
HashWithIndifferentAccess[T] < Hash[(String | Symbol), T]
```

みたいな感じの方が正しいのではないか説