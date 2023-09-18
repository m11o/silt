## Cloud Storage

version管理ができる
IAMロールでアクセス制御もできる
ライフサイクルポリシー
  - 一定期間後に自動的に削除する
  - 設定した日に削除する

ストレージ制限なし
成獣どこからでもアクセス可能
low latency
地理的な差もない

バッチ処理もオッケー
Storage Transfer serviceで大容量データを送信できる
Google Cloudプロダクト間でのデータ転送も可能
  - BigQuery, Firestore etc..

### Standard Storage

利用頻度の高いものに使用する

### Nearline Storage

利用頻度の低いデータに最適
月に1回ぐらいの頻度が最適

### Coldline Storage

利用頻度の
90日に1回ぐらいの頻度が最適

### Archive Storage

年に一回アクセス
365の最低保存期間がある

## Cloud SQL

ソフトウェアのアップデートや保守も不要

## Cloud Spanner

RDB
**水平スケーリング可能**
強整合性
SQL対応

向いているプロダクト

- テーブル結合
- セカンダリーインデックス
- 組み込みの高可能性
- グローバル整合性
- 2てらバイトを超える膨大なデータ数
- 1秒あたり数万回以上の読み取り・書き込みがある場合

## Firestore

NoSQL
ドキュメント指向
水平スケーリング可能

## Bogtable

bigdata 用のNoSQL
運用目的・分析用どちらでも使用可能

用途
- 1テラバイトを超える構造化データ
- データが高スループットで高速移動・急速に変化する場合
- NoSQLを使う場合

public IP: 34.162.93.190



