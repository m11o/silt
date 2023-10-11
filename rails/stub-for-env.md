# ENVをstubしたい

ENVを普通にstubしてしまうと、他のENV['other_key']みたいなものもstubされてしまい、テストが通らなくなる。
なので、 `and_call_original` を使用して、他のkeyはstubしないようにする

```ruby
allow(ENV).to receive(:[]).and_call_original
allow(ENV).to receive(:[]).with('key').and_return('value')
```
