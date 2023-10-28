# APM

Application Performance Management

何を見ればいいのか

## APMの事例

リクエスト数と応答速度をみることがある

スパンが短いとN+1があるかも
スパンが長いとインデックスが足りないかも

rubyはspanがでないので、手動でspanが登録できるようにする
DDtrace::tracing.trace

apm tracable gem
push通知のプレビュー画面にあると良さそう