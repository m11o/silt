## Resource hierarchy

Organization node -> Folders -> Projects -> Resources
ポリシーは上から下に適用される
なので、同じ設定を同じ階層で持つ必要はない

フォルダを使用するためには、Organization nodeの管理者権限が必要

## IAM (Identity and Access Management)

### 基本ロール

範囲の広いロール
オーナー、編集者、閲覧者、課金管理者がある
複数ユーザーが共同作業する場合は、基本ロールでは範囲が広すぎる
Google Cloud プロジェクト全てのリソースに影響する

### 事前定義ロール

事前に定義されており、ロールを適用できる場所も内容も定義がすでに行われている
プロジェクト内の特定のサービスに適用される

### カスタムロール

プロジェクトレベルかOrganizerレベルにのみ適用できる
フォルダにはできない
細かく定義できる。
インスタンスの起動や停止、閲覧はできるが、構成を変更することができないなど

## サービスアカウント

サービスアカウントは、サービス(Computer Engine VM など)がGoogle Cloud PlatformのリソースにアクセスするためのGoogleアカウント

## Cloud Identity

Google WorkspaceのアカウントをGoogle Cloud Platformに接続することができる

## Google Cloudの操作方法

1. Cloud Console
    - GUIで操作できる
    - ブラウザベース
2. Cloud SDK
    - CLIで操作できる
3. Cloud Shell API
    - ブラウザからCLIを操作できる
    - API経由で操作する
4. Cloud Console Mobile App
    - スマホから操作できる
    - 課金情報、予算超過、様々な指標の確認もできる

## テスト

下位のIAMポリシーは上位のIAMポリシーを上書きできる

