# Google Cloud のデータベース

- Alloy DB (PostgreSQL 完全互換)

## Cloud SQL

マネージドデータベースサービス
AWSでいうとRDSに相当

MySQL, postgresql, SQL Server, Oracle

## Alloy DB

- 高い可用性
- 高いスケーラビリティ
  - ストレージが自動的に拡張される
  - 容量を設定しておく必要はない
- インテリジェント
- 優れたパフォーマンス

## Cloud Spanner

- フルマネージド
- スケールをほぼ無制限でできる
- ACIDトランザクションをサポート
- ノード数の変更時のダウンタイムもなし(メンテナンスもなし)
- 無停止でスケールアウト・スケールインが可能

Leaderノードを立ち上げると、別のzoneにレプリカが自動的に立ち上がる
Leaderノードのzoneで障害が起きると、別のzoneのノードがLeaderノードになる

DynamoDBが少し近い
ただし、key-valueデータベース

拡張性
別のノードのLeaderノードが追加される

Granular instance sizing
1node => 1000 PU (今まで)
0.1node => 100 PU (新しく追加, about $80/month) 


https://cloud.google.com/spanner/docs?hl=ja

## Cloud Firestore

サーバーレスのデータベース

ネイティブモード
-> アプリからAPI経由で直接通信できる

Datastoreモード
-> ドキュメントDBのように使える

## Bigtable

ワイドカラムストア(NoSQLの一種)
key-valueストアに似ている(valueの部分が幅を持って保持することができる)

## Memorystore

インメモリストア -> cacheの用途で使用できる
ElastiCacheのようなもの

## Database Migration Service

AWS => 汎用的なDBから汎用的なDB
GCP => 同種のDBの移行を対象(MySQLからMySQL, PostgreSQLからPostgreSQLみたいな)

