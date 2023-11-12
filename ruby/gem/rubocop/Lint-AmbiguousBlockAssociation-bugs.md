# Lint/AmbiguousBlockAssociation

```ruby
expect { subject }.to change { article.reload.title }.to('title')
```

上記コードはエラーにならないが、

```ruby
expect { subject }.not_to change { article.reload.title }
```

このコードだとエラーになってしまう。

やってること同じだろーーーー