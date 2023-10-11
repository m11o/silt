# パラメータをfilterする方法

ActiveSupport::ParameterFilterを使用する

```ruby
ActiveSupport::ParameterFilter.new([:password]).filter({password: 'password'})
```
