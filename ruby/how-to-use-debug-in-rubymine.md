# Ruby mineでdebugする方法

Ruby Mineでは `ruby-debug-ide` gemを使用してdebugを行う。
また、このgemがbundleされている必要がある。つまり、Gemfileにruby-debug-ideを追加する必要がある。

```ruby
gem 'ruby-debug-ide'
```

また、 `io-wait` も0.2.0以上を使用する必要があるため、もし、エラーが発生する場合には、以下のコマンドを実行し、updateする

```bash
bundle update --conservative io-wait
```


```ruby
  gem 'ruby-debug-ide', require: false
  gem 'io-wait', '~> 0.3.0', require: false
```