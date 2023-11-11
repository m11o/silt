# アプリのスケーラビリティ、高可用性、セキュリティ

federated identityを使用すれば、別に認証の実装は必要ない
管理する必要があるのは、
- コード
- 依存関係のあるライブラリ
- 環境変数

デブロイが可能かどうかをチェックするために、ヘルスチェックエンドポイントを用意する必要がある

カナリアデプロイやブルーグリーンデプロイを経て、あまりユーザーに影響がでない方法でデプロイする必要がある

## federated identityとは

Firebase authenticationみたいなやつかな
OAuth2

## ストラングラーパターンとは

システムを移行するための戦略の一つ
既存のアプリと新規のアプリを振り分けるレイヤーを用意して、少しずつ新しいアプリに移行していく手法
ref: https://atmarkit.itmedia.co.jp/ait/articles/2001/23/news003.html

# Google Cloud SDK, Cloud クライアントライブラリ、Firebase SDK

gcloud コマンドの認証情報を再利用される

- gcloud
  - GCPリソースの作成・管理ができる
- bq
  - BigQueryの操作
- gsutil
  - Cloud Storageの操作

## API Explorer

REST APIリファレンスドキュメントページで使用できるツール
コードを記述せずにGoogle APIメソッドを試すことができる
実際のデータに基づいて動いているので、変更・削除を試すときは注意が必要

GCPコンソールから使用可能

## gRPCとは

REST APIとの違い

# Cloud データ

## Cloud Storage

S3みたいなやつ

## Cloud Datastore

Managed NoSQL document database
GQLで操作する？

今はFirestore のdatastoreモデルが後継として存在する
Firestoreネイティブモードはクライアントから直接アクセスできる

全てのプロパティにインデックスがはられている
しかし、複数のカラムで検索したい場合には、複合indexを貼る必要がある

組み込みインデックス -> 自動で作成されるインデックスのこと
複合インデックス -> 手動で作成する必要がある

### 祖先関係と祖先キー関係の違いについて調べる



## Bigtable

## Cloud Spanner

idは自動採番されない
シーケンシャルなIDを使用しない
-> IDによって振り分けるため、シーケンシャルなIDだと一つのノードに集中してしまい、パフォーマンスが低下する
-> UUIDなどをIDとして使用する

## Cloud SQL

### Cloud SQL Proxyで接続する

Cloud Compute VMからDBに接続する推奨方法

## アプリケーションサーバーとウェブサーバーの違い

ウェブサーバーは静的なコンテンツ(HTML, 画像)などを配信するサーバー
アプリケーションサーバーは、動的なコンテンツ(個人情報やリアルタイム情報)などを配信するサーバー



# Cloud Storage

99.9999999の耐久性(データを保持する割合)
可用性とは、サービスが障害やエラーなく稼働できる度合い

- standard
- nearline
- coldline
- archive

のストレージタイプが存在する

## multi-regional

99.5%の可用性

## reginal


## nealine

長時間のバックアップ
99%可能性

## coldline

## buchet

サービス全体でグローバル
DNS命名規則にも従う

ユーザーに近いリージョンに保存する
ACLはバケットとオブジェクト両方に設定できる
IAMもある

IAMはgoogle cloud全体で使用できるが、ACLはgoogle cloud storageのみの概念


署名つきドキュメントは制限をつけることができる

ajaxでXHRコールバックで進捗を確認する場合、停止、再開をしてはいけない。中途半端なデータが残ってしまう

CRC32CやMD5で検証している(検証する必要がある)(フロントもバックエンドも)
そもそもHTTPSで通信する必要はある

gzipのものをあげてもメリットがない(圧縮ファイルが意味ないってことかな？)

## 型指数バックオフ について調べる

この記事を読む感じは、再実行までの間隔を増やしながら、再実行するみたいなことを自動的にすることかな？
ref: https://cloud.google.com/storage/docs/retry-strategy?hl=ja
