# Q. 3rd party製のアプリケーションからCloud Loggingにログを出力する方法

answer: Cloud Logging Agent

## Cloud Logging Agentとは

今はOps Agentになっている

### 概要

Logging エージェントは、VM インスタンスや選択したサードパーティ ソフトウェア パッケージから Cloud Logging にログをストリーミングします。

すべての VM インスタンスで Logging エージェントを実行することをおすすめします。

サードパーティ用なのか？

マネージドサービスの場合は、自動的にログが送信される
VMは、loggingエージェントを入れることでログを送信することができる

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

# Q. アプリケーション間のバランスを取りたい

internal load balancer

## Load Balancerの種類

HTTP(S)ロードバランサー = アプリケーションロードバランサー
それ以外 = ネットワークロードバランサー

- 外部・内部HTTP(S)ロードバランサー
- 外部・内部TCP/UDPロードバランサー
- 外部・内部TCP/SSLプロキシロードバランサー

以下、ざっくり概要

- Global HTTPS load balancing
    - リージョン間の負荷分散
- Global SSL proxy load balancing
    - SSLトラフィックの負荷分散用
- Global TCP proxy load balancing
    - そのほかのTCPトラフィック分散用
- Regional network load balancing
  - 
- Regional internal load balancing
    - プロジェクト内部のトラフィックの負荷分散

# Cloud CDNのベストプラクティス

- 静的なファイルのキャッシュに使用する
- 個人情報には使用しない
- ネガティブキャッシュにも使用可能
- セキュリティ強化のためにCloud Armorを使用する

# IAMは必要最小の権限のみを付与する原則

POLP

# Cloud Storage の設定時に必要なこと

- ストレージクラス
- Global unique name
- location type
- アクセス権限(IAM Policy or ACL)
  - IAMはシステム全体に適用できる
  - ACLはオブジェクトごとにユーザー単位で権限を付与できる（細かく設定した場合）

# Artifact Registry

限定公開のコンテナイメージを管理するためのサービス

# Customer Managed Encryption Key(CMEK)

Googleが管理する鍵よりも細かく制御する必要がある場合に顧客管理の暗号鍵を使用することができる

# S3 から cloud storageへの移行

Storage Transfer Serviceを使用する

## 別の選択肢にあった商品

- Cloud Dataproc
  - フルマネージドで自動化されたビッグデータ用オープンソース ソフトウェア
  - HadoopやApache Sparkなどのオープンソースのフレームワークを使用して、データ処理や分析を行うためのサービス
  - データ分析を主に行う
- BigQuery Data Transfer Service
  - BigQueryにデータを移行するためのサービス
  - あらかじめスケジュールを決めて、それに基づいて実行される
  - データ分析を主に行う
- Transfer Appliance
  - データをGoogle アップロード施設へアップロードし、Cloud Storageに転送する
  - 転送用のリアルデバイス
  
# k8sのnamespace

クラスター内にもつ
一つのクラスター内に複数のネームスペースを持つことができる
それぞれの名前空間が論理的に独立している

セキュリティ的にもパフォーマンス的にも良い

# subnet and VPC

- VPCはグローバルリソースである
  - 一つ以上のサブネットを持つ
- サブネットはグローバルリソースではない
  - リージョンリソース
- VPCネットワークは安全に複数の環境と接続することができる with Cloud VPN or Cloud interconnect
  - interconnectはVPC同士のネットワーク間のデータのやり取りを行うことができる

# MIG(Managed Instance Group)

Compute Engine のインスタンスの集まり
同じVMのグループを一括操作できる
自動スケーリング、自動修復、リージョン（マルチゾーン）デプロイメント、自動更新をサポート

##  rolling update

基本的なローリング アップデートとは、すべてのインスタンスが意図する最新の構成に更新されるまで、MIG のすべてのインスタンスに段階的に適用されるアップデート。
最新のインスタンスはスキップされる

影響を受ける各ゾーン内では 1 つのインスタンスだけがオフラインになり、MIG は更新中にゾーンごとに 1 つの追加インスタンスのみを作成

maxSurge: 自動更新中に MIG が targetSize を超えて作成できる新しいインスタンスの数を構成
maxUnavailable: 自動更新中に常時利用できないインスタンスの数を構成

# GKEのオートスケール

5種類

- 水平Podオートスケーリング
  - Podの負荷に応じてPodの数を自動的に増減させる
  - Kubernetesの機能(GKE独自ではない)
- 垂直Podオートスケーリング
  - Podのリソース(CPU, Memory)を自動的に増減させる
  - 性能向上のためのスケーリングというよりは、リソース使用率の最適化の性格が強い印象
- 多次元Podオートスケーリング
  - 水平・垂直方向のスケーリングを両方向にスケーリングできるようにする
- クラスタオートスケーリング(CA)
  - Pod のデプロイ要求に応じてノードを増やしたり、また逆に、遊んでいるノードを削除
  - GKE の独自機能というわけではなく、Kubernetes のコア + 各プロバイダ用の実装 という設計
  - **クラスタとは？Podやサービスとの違いを調べる**
- ノード自動プロビジョニング(NAP)
  - CA の拡張版 (Proposal) で、Pod の要求する CPU やメモリ、GPU、Taint に基づいてノードプールを用意
  - CA がノードを増減するのに対し、NAP はノードプール自体を作ったり消したりする

ref: https://tech.quickguard.jp/posts/gke-autoscale-overview/

# KubernetesのPod

実行可能な最小単位
でもPodは必ずNode内で実行されるので、１つのpodが最小構成なら1つのnodeも最小構成のはず

# Cloud CDNのdefault TTL

デフォルト: 1時間
最大: 1年

# Cloud Functionのランタイム

- Node.js
- Python
- Go
- Java
- .NET
- Ruby
- PHP

# Cloud Spanner v.s Cloud SQL

Spannerは水平スケールが可能
SQLは垂直スケールのみ

# k8sのnodeの数を増やす方法

gcloud コマンドを使用するらしい
以下参照: https://cloud.google.com/kubernetes-engine/docs/how-to/resizing-a-cluster

# AIのpre-trained modelにアクセスする方法

- cloud client libraries
- REST API

でアクセスできるみたい
参照: https://cloud.google.com/products/ai

# VPC network peering

VPCネットワーク間での通信を可能にする
Each VPN endpoint supports up to 3 Gbps, so two will be sufficient.
（それぞれのVPNエンドポイントごとに3 Gbps サポートしている）
なので、5Gbpsの通信を行うには、2つのVPNエンドポイントが必要

ref: https://cloud.google.com/vpc/docs/vpc-peering?hl=ja

## その他の接続方法

- Cloud VPN
  - オンプレミスとの接続
  - 暗号化されていない情報に対してデータセンターをクラウドに拡張
- Cloud DNS
  - ドメインとの接続 
- Cloud Interconnect
  - オンプレミスとの接続
- Hybrid Connectivity
  - オンプレミスとの接続

# Cloud Armor

DDoS攻撃などからアプリケーションを保護するためのサービス
クロスサイト スクリプティング（XSS）、SQL インジェクション（SQLi）も

アプリケーションを保護というよりは、デプロイを保護する

ref: https://cloud.google.com/armor/docs/cloud-armor-overview

# Compute Engineの状態ごとの料金

- Stopped
  - ディスクのみ課金
- running
  - ディスクとCPU, memoryの課金

# Cloud Datastore

プロパティは複数の型の値を持つことができるため、同じプロパティであってもエンティティによって値の型が異なる場合があります
数値 ID を自動生成したりする代わりに、作成したエンティティに独自の数値 ID を手動で割り当てることができます
  -> 競合を回避する唯一の方法は、allocateIds() メソッドを使ってアプリケーションで ID のブロックを取得すること

# 管理可能な観点で管理できる範囲が広い順

IaaS > CasS > PaaS > FaaS > SssS

# Bigtable

- key/value store
- 各テーブルにインデックスは１つのみ
- 行は、行キーの（最小バイト文字列から最大バイト文字列まで）辞書順に並べ替えられます
- すべてのオペレーションは行レベルでアトミックに実行

# Dataflow

- データ処理のためのサービス
- データのバッチ処理とストリーミング処理の両方をサポート
- データ ストリーミングと機械学習によるリアルタイムの分析情報と有効活用

# Cloud Storage Lifecycle

lifecycleを設定することによって、一定期間経ったstorageのstorageクラスを変更したりできる

# Cloud SpannerのID指定方法

キーが単調増加、単調現象する場合には、ホットスポットになりやすい
Timestampをprimary keyとして使用することもhotspotになりやすい

# Cloud Pub/Sub v.s Cloud Task

大きな違いは呼び出しについて
pub/subは、暗黙的呼び出し
taskは、明示的呼び出し

どちらもwebhookからのpushを受け取ることができ、実行を遅らせることができる
ただし、明示的な実行か、暗黙的な呼び出しかの違いがある

ref: https://cloud.google.com/pubsub/docs/choosing-pubsub-or-cloud-tasks?hl=ja

# Service Account

ユーザーではなく、アプリケーションや Compute Engine インスタンスなどのコンピューティング ワークロードで通常使用される特別なアカウント

アプリケーションはサービス アカウントを使用して、認可された API 呼び出しを行います。
これは、サービス アカウント自体として認証されるか、ドメイン全体の委任により Google Workspace または Cloud Identity ユーザーとして認証されます

# Bucket name

- 小文字のみ使用可能
- グローバルユニーク
- 英数字とハイフン、アンスコ、ピリオドを使用可能
- IPアドレスは不可
- googで始まる名前は不可
- googleやそれに似た言葉(g00gleなど)は不可

# gsutil mb

make bucket
バケットを作成する

# Shielded VM

- シールドされた VM は、VM の起動時に、VM の完全性を検証し、VM が正当なものであることを確認します
- Compute Engine VM インスタンスの検証可能な整合性を提供
  - インスタンスがブートレベルまたはカーネルレベルのマルウェアやルートキットによって改ざんされていないことを確認できます

# Calculatorでコストを試算可能

ref: https://cloud.google.com/products/calculator

# グローバルリソース、リージョナルリソース、ゾーンリソース

## グローバルリソース

- アドレス
- イメージ
- スナップショット
- グローバルインスタンステンプレート
- Cloud Interconnect の接続
- VPC
- ファイアーウォール
- ルート
- グローバルオペレーション

## リージョナルリソース

- サブネット
- Cloud Interconnectのアタッチメント
- プレースメントポリシー
- リージョナルインスタンステンプレート
- リージョンマネージドインスタンスグループ
- リージョン永続ディスク

## ゾーンリソース

- 永続ディスク
- インスタンス
- マシンタイプ
- ゾーンマネージドインスタンスグループ
- ゾーンオペレーション

# preemptible instance

Compute Engine の余剰の容量を利用する機能であり、使用できるかどうかは利用状況に応じて変わります。
標準 VM の料金よりもはるかに低価格

# GKE PersistentVolume

クラスタ内の耐久性に優れたストレージの管理に使用
ノードの再起動やPodの再起動によってもデータが保持される
-> Podとは独立して存在するクラスタ

## ReplicaSet

Podがどんな時も指定した数のpodが稼働することを保証する

## Deployment

ReplicaSetを管理するためのリソース
ReplicaSetの上位概念

## StatefulSet

StatefulSetはステートフルなアプリケーションを管理するためのワークロードAPI
DeploymentとPodのセットのスケーリングを管理し、それらのPodの順序と一意性を保証

ステートフルなアプリケーションや、Compute Engineの永続ディスクなどの永続ストレージにデータを保存するクラスタ化されたアプリケーションをデプロイするために設計

## DaemonSet

全て(またはいくつか)のNodeが単一のPodのコピーを稼働させることを保証
すべてのノードまたは特定のノードで実行する必要があり、ユーザーの介入を必要としない継続的なバックグラウンドタスクをデプロイするのに便利

- Nodeがクラスターに追加されるとき、PodがNode上に追加されます。
- Nodeがクラスターから削除されたとき、それらのPodはガーベージコレクターにより除去

# Cloud run の最大コンテナ数

デフォルト設定の最大コンテナ数は、100個
ref: https://cloud.google.com/run/docs/configuring/max-instances?hl=ja#setting
