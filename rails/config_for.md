# config_for

initializersなどに記述されているyamlファイルから設定値を取得するためのメソッド

設定値が記述されたYAMLファイル
```yaml
# config/exception_notification.yml:
production:
  url: http://127.0.0.1:8080
  namespace: my_app_production

development:
  url: http://localhost:3001
  namespace: my_app_development
```

上のYAMLファイルを読み込んで、設定値を取得している
```ruby

# config/environments/production.rb
Rails.application.configure do
  config.middleware.use ExceptionNotifier, config_for(:exception_notification)
end
```

second parameterに、Rails.envなどのどの環境かを渡すこともできる

TODO: ActiveSupport::OrderedOptionsに依存しているので、reviewで通った後にこのメソッドにも型を追加する