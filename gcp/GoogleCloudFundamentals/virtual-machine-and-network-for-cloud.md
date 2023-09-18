## virtual private cloud network

### VPC

VPCはグローバル
パプリッククラウドとプライベートクラウドの両方を享受できる
(スケーラビリティ、利便性、データ隔離)

複数リージョンのVPCを作成することもできる
サブネットのサイズはIPアドレスの範囲を広げることで拡張できる
-> 構成済みの仮想マシンが影響を受けることはない

サブネットはリージョン単位
複数リージョンを跨ぐことができる

負荷分散にも対応

ルーティングテーブルは組み込み
  -> ルータのプロビジョニングも管理も不要

ファイアウォール
  - プロビジョニング・管理が不要
  - グローバル分散型

### Compute Engine

IaaS

### プロビジョニング

必要に応じてネットワークやコンピュータの設備などのリソースを提供できるように予測し、準備しておくこと
-> 必要な時に必要な分だけ、オンデマンドでリソースを割り当てます

## Cloud Load Balancing

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

## Cloud DNS and Cloud CDN

CDNは別のCDNを使用しても、Google Cloud CDN Interconnectに含まれていれば、そのまま利用できる

## connect to Google VPC

５つの方法

- VPN
  - セキュリティ上の懸念や帯域幅の信頼性が問題
- Direct Peering
    - 同じパブリックデータセンターにルータをおき、ネットワーク間のトラフィック交換に使用する
- Carrier Peering
    - サービスプロパイダ経由でGoogle Cloudプロダクトに接続する
    - Google SLAで保証されない
      - SLA(Service Level Agreement)
        - サービス提供者とサービス利用者の間で、サービスの品質を保証するための契約
- Dedicated Interconnect
    - 相互接続
    - Google への直接プライベート接続を利用することができるサービス
    - 仕様を満たしているならSLAを適用できる
- Partner Interconnect
  - 仕様を満たしているなら、SLAを適用できる
  - データセンターの物理的な場所に関係なく、Google Cloud Platformに接続できる

10.138.0.0/20
10.154.0.0/20

inner 10.154.0.2
outer 34.147.192.148

ping はICMPプロトコルを使用している