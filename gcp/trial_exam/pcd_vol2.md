# Cloud Monitoring

Google Cloud、Amazon Web Services（AWS）、ホストされた稼働時間プローブ、アプリケーション インストゥルメンテーションから指標、イベント、メタデータを収集

# Cloud Trace

アプリケーションからレイテンシ データを収集し、Google Cloud Console に表示
アプリケーション内でリクエストがどのように伝播するかを追跡し、ほぼリアルタイムでパフォーマンスに関する分析情報を受信

# App Engine Standard v.s Flexible

Standardは、サーバーのことは全く見えない、もしくは意識しなくても良い
また、スケーリングも自動で行ってくれる

Flexibleは、Compute Engine上にcontainerを作成する
なので、管理が容易なCompute Engineである
Dockerfileで管理することができるので、自由度が高い

# Kubernetes でデフォルトで設定されるnamespace

- default
- kube-system
- kube-public
- kube-node-lease

# Transfer Appliance

物理デバイス
大量のデータを移して、Cloud Storageに転送する

## その他のTransfer Service

| Service                             | ユースケース                          |
|-------------------------------------|---------------------------------|
| Transfer Appliance                  | 帯域幅が限られている場合<br>物理的な転送ができる場合<br>時間に余裕(20日)がある場合 |
| Storage Transfer Service            | 100TBを1Gbpsで10日<br>クラウド間で転送する場合 |
| Storage Transfer Service For On Premises 	 | 数PB<br>十分な帯域幅があるとき |
| gsutil                                    | 1TB未満のデータ<br>-mオプションによるマルチスレッド化<br>parallel composite upload |

# Machine type family

## General-purpose machine types

## Compute-optimized machine types

コンピュータ計算をいっぱいする系の用途に使用するマシンタイプ
ゲームなど

## Memory-optimized machine types

メモリをいっぱい使う系の用途に使用するマシンタイプ
redisサーバー
シミュレーション

## Accelerator-optimized machine types

GPUを使う系の用途に使用するマシンタイプ
AI モデリング
超並列コンピューティング

## cloud shell

5GBの永続ディスクがユーザーごとに割り当てられる
これらはセッションを横断して保持される

# Cloud BigTable のレプリケーション

１つのインスタンスに複数のクラスタを作成することで実現できる
その上で、クラスタ間で自動的にsyncしてくれる
