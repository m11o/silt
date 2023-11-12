# IAMのコンセプト

ユーザーを作って、それぞれにどのリソースにアクセスできるかを設定できる

サービス.リソース.動詞の順番で権限をつけれる
事前定義ロールもある

権限は最小にする
->最小権限の原則

階層ごとにロールを設定することができる
階層: Organization > Folder > Project > Resources

## IAM ポリシー

プロジェクトにassignすることができる

## API

- setIAMPolicy
- getIAMPolicy
- testIamPermissions

## ベストプラクティス

外部鍵の管理
- IAM API
- gcloud

## IAP

Identity-Aware Proxy
クラウドアプリへのアクセスを制御できる
例えば、　HTTP経由で接続を制御

VPNが不要-> なぜ？

ファイアウォールとロードバランサを適切に設定する

これはユーザーがcloud runなどにアクセスするために認証が必要な場合に使用する

https://cloud.google.com/blog/products/identity-security/account-authentication-and-password-management-best-practices?hl=en

## サービスアカウント

個々のエンドユーザーではなく、アプリケーションやVMインスタンスに属している特別なGoogleアカウント。
サービスのGoogle APIを呼び出すためには、サービスアカウントを使用する

## Firebase authentication

クライアントアプリケーションが認証を必要としている場合に使用する

# Cloud Pub/Sub

## コンセプト

publisher -> pub/sub -> subscriber
自動スケーリングあり
ポイントToポイント通信を避けるために使用する
-> 疎結合にするため

### pull subscription

バッチ処理が可能

### push subscription

pub/subがhttpプロトコルでsubscriberに送る
-> subscriberはhttp statusを返す

cloud functionが受け取って、処理を実行できる

## ユースケース

pub/subを使用することによって、サービスを疎結合にすることができる
順番を持たせて処理することができる

# AIの追加

# Cloud Functions

## deploy方法

- 直接コードを書く
- cloud storageからzipファイルを読み込む
- cloud source repositoryから呼び出す

### 非同期イベント

バックグランド関数

### 同期イベント

HTTP関数

### エラーが発生した場合

stackdriver error reportingにエラーがキャプチャされる

### cloud platform console

cloud platform consoleで関数の呼び出し回数、実行時間、メモリ使用量に関するメトリックスを取得することができる

# Cloud Endpoint

- interfaceの定義
  - OpenAPI, gRPC
- 認証・認可
  - Firebase、google認証, Auth0
  - JWT署名されているかも検証している
- APIの拡張・バージョニング
- ロギング

↑ここら辺を全て管理できる

api_name.endpoints.projectId.cloud.google
