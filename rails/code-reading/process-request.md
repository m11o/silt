# リクエストを受け取る仕組み


1. 
1. Rack::Runtime
  - リクエストの実行時間を計算してheaderに詰める作業を行っている
1. Rack::MethodOverride
  - post以外は次のmiddlewareへ
    - ここのpostはdeleteなどもあるのかどうか
    - ヘッダーの `_method` を確認しているので、DELETEなども含まれていそう
  - 逆にPOSTで送られてくるDELETEやPUTなども_methodから取得する
1. ActionDispatch::RequestId
  - X-Request-Idにリクエスト単位でuuidを設定する
  - 6.1からinitializeの引数にheaderを渡しているので、修正する
1. ActionDispatch::RemoteIp
  - remote_ipを計算して取得する
  - ActionDispatch::Request#remote_ipで呼び出しされる(ActionDispatch::RemoteIp::GetIp)
  - これも型をかけそう

# memo

```ruby
params.require(:article)
```

paramsメソッドに型がないので、requireメソッドがKernelのrequireと勘違いされている