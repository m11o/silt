# サーバーレスとは

- オートスケール
  - 負荷に応じてスケールイン・スケールアウトする
- フルマネジド
  - 開発に集中できるか
  - ビジネスの開発に集中する
- スピーディなデプロイ

オンプレ -> クラウド -> サーバーレス
の順でやることが減る

# GCPのサーバーレスファミリー

## Cloud Run

コンテナを実行することができるアプリケーションを起動・スケールできるサーバーレスサービス
libraryはコンテナ内に設定できるので、割と自由の設定可能

対照サービス => k8s
k8sのようにクラスタ管理は必要なし

課金
- HTTPリクエスト処理時間に応じた課金
- インスタンス時間に応じた課金

アーキテクチャ
- Revisionに環境変数などの設定値が含まれてdeployされる

IAMを挟んで、認証制御を行うことができる

ロードバランサーを挟むことで、同じIP(URL)に対して近いリージョンから返すことができる

Cloud Run jobの場合
Project => Job => Task => Container

## Cloud Function

FaaS(Function as a Service)のサービス
イベントやHTTPリクエストを受けて関数を実行する

第２世代とCloud Runは類似している
違いは `コンテナ` か `関数`　かみたいな違いがある
=> ライブラリ等を使用したりする場合にはCloud Runかな

## Cloud Build

フルマネジドのCIプラットフォーム
Github等から変更をトリガーにして、ビルド・テスト・デプロイを行うことができる
デプロイは Cloud Deployがある

## Cloud Deploy

デプロイできる
後ほど記事を共有いただく

## Cloud Workflows

HTTPリクエストをトリガーにするのが基本?
APIコール・認証などをYAMLに記述

## Eventarc

GCP内のイベントをトリガーにすることができる
Pub/sub経由でカスタマーアプリケーションをトリガーにすることができる

## Cloud Tasks

大量のタスクを非同期で処理するための分散タスクキュー

## Cloud Scheduler

cron
日時などでタスクをキックすることができる

## Cloud Workstation

クラウドのIDE

# ハンズオン

色々やった
visualizerすごい

ingress ルール
-> ロードバランサーの設定
    -> VPC ネットワーク内部から発信されていないリクエストを拒否
-> --allow-unauthenticated フラグを使用して metrics-writer Cloud Run サービスをデプロイしました。これにより、サービス URL がインターネット上で公開され、誰でもあなたのサービスに接続することができます。

# OpenTF

terraformのgoogle cloud版