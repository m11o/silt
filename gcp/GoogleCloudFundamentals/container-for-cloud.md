# App Engine

PaaS
IaaSはハードウェアの仮想化を使用する
-> 需要が高まると時間がかかる可能性がある

コンテナ
PaaSのようにスケールできるし、IaaSのように柔軟性にも優れている
固有のパーティションとハードウェアのみアクセスする
hostに必要なもの
- コンテナ
- ランタイムを実行するためのOSカーネル

# Kubernetes

Podは最小単位
-> １つ以上のコンテナをラップして、ノードにdeployしたもの

更新はrolloutコマンド行う方が良い
理由は、更新中に問題が発生した場合に元に戻せるから
新しいpodが作成された後に古いpodが削除される

ServiceはPodの集合体
-> 論理的なセットとPodへのアクセスに使うポリシーを定義する
-> Podに安定した固定IPを提供する
GKE(Google Kubernetes Engine)はKubernetesのマネージドサービス
-> GKEはネットワークロードバランサとして作成されて、IPに到達したクライアントはserviceの背後のPodにルーティングされる


# Google Kubernetes Engine

Compute Engineの上にKubernetesを構築したもの

# hybrid cloud and multi-cloud

オンプレミスサーバーとクラウドを組み合わせたもの

# Anthos

分散システムとサービスを管理
ハイブリッド・マルチクラウド環境を管理