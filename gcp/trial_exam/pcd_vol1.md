# Q. 3rd party製のアプリケーションからCloud Loggingにログを出力する方法

answer: Cloud Logging Agent

## Cloud Logging Agentとは

今はOps Agentになっている

### 概要

Logging エージェントは、VM インスタンスや選択したサードパーティ ソフトウェア パッケージから Cloud Logging にログをストリーミングします。

すべての VM インスタンスで Logging エージェントを実行することをおすすめします。

サードパーティ用なのか？

## Cloud Trace

レイテンシーデータなどを修習し、コンソールなどに出力する

Cloud Run、Cloud Functions、App Engine スタンダード アプリケーションはすべて自動的にトレース

それ以外は、ライブラリを使用してトレースを追加する必要がある

## Cloud Debugger

廃止されている
代わりにオープンソースのSnapshot Debuggerを使用可能

## Cloud Error Reporting

Error Reporting は、運用中のクラウド サービスで発生したクラッシュを計数、分析、集計するためのサービスです。

問題が発生するとエラーを通知してくれる

# Q. Cloud Storageのデータの変更のコンプライアンスをリアルタイムで検証する方法

Cloud Functions
-> リアルタイムで実行することができる

Storage自体に、権限等を付与することはできるが、コンプライアンス検証をリアルタイムで行うことはできない
