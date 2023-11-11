# HTTPとTCPの違い

TCP/ID階層モデルにおけるレイヤーの違い
HTTP: アプリケーション層
TCP: トランスポート層

## HTTPリクエストを送信する方法

1. HTTPプロトコルで送りたいデータを作成する
2. 下位層であるTCPにデータを渡し、 `TCPヘッダー` が付与される
3. さらに下位層であるネットワーク層にデータを渡し、 `IPヘッダー` が付与される
4. さらに下位層であるデータリンク層にデータを渡し、 `イーサネットヘッダ` が付与される
5. ネットワークを通してデータを送信する

ref: https://qiita.com/shinobear01/items/a9d0cdc1ef2639708160

rackはHTTPリクエストとレスポンスをwrapしているっぽい
rack↓

> By wrapping HTTP requests and responses in the simplest way possible, it unifies and distills the bridge between web servers, web frameworks, and web application into a single method call.
