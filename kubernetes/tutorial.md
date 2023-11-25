# Kubernetesクラスター

## ２つの構成要素

- マスター
  - クラスターの管理を行う
  - APIサーバー、スケジューラー、コントローラー、etcd
- ノード
  - ユーザーのアプリケーションを実行する
  - kubelet、kube-proxy、コンテナランタイム
  - マスターと通信するためのkubeletとkube-proxyが必要

# Kubernetes Pod

- Kubernetesの最小単位
- 一つ以上のコンテナを含む
- 以下のものが含まれる
    - 共有ストレージ(ボリューム)
    - ネットワーキング(クラスターに固有のIPアドレス)
    - コンテナのイメージバージョンや使用するポートなどの、各コンテナをどう動かすかに関する情報
- Podはワーカーノード上で実行される
- ノードはKubernetesではワーカーマシンであり、クラスターによって仮想、物理マシンのどちらであっても構わない

# Kubernetes Service

- Podの集合に対するアクセス方法を定義する
- Podは短命であるため、PodのIPアドレスは変化する
- ServiceはPodのIPアドレスが変化しても、ServiceのIPアドレスは変化しない
- ServiceはPodの集合に対してロードバランシングを行う

# Kubernetes ローリングアップデート

- ローリングアップデートは、アプリケーションの新しいバージョンをデプロイするための方法

ref: https://kubernetes.io/ja/docs/tutorials/kubernetes-basics/