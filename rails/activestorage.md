# detach方法

detachメソッドでアタッチメントを削除できる。

```ruby
class User
  has_one_attached :logo
end

user.logo.detach
```
