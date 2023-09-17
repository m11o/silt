## Cloud Computing

google cloudはコンテナベース
第１波: Collocation
第２波: Virtualization (仮想化)
第３波: Containerization

仮想化クラウドだと、ビジネスを迅速化できない

サーバー仮想化だとOSの上にさらに仮想OSを立ち上げる必要がある

## IaaS, PaaS, SaaS

### SaaS

インターネット経由でエンドユーザーがアプリケーションを利用する
=> Google Apps(Gmail, Google Documents), Salesforce, Dropbox, DocuSign

### IaaS

物理的なコンピューティングストレージ、ネットワーク機能を物理センターと同等のリソースとして仮想的に整理したもの
事前に割り当てたリソースについて料金が発生する

e.g. Google Compute Platform, Amazon EC2, Microsoft Azure

### PaaS

アプリに必要なインフラへのアクセスを提供する
実際に使用したリソースについて料金が発生する

e.g. Cloud Run, Cloud Functions

|  | IaaS | PaaS | SaaS |
| --- |------|------|------|
| アプリケーション |      |      | ○    |
| ミドルウェア |      | ○    | ○    |
| OS | ○    | ○    | ○    |
| ハードウェア | ○    | ○    | ○    |
| ネットワーク | ○    | ○    | ○    |

## Google Cloud ネットワーク

location -> region -> zoneの順番

### location

北米・南米・ヨーロッパ・アジア・オーストラリアの5つの主要地域に存在する
各locationは複数のregionとzoneに分割される

### region

regionには複数のzoneが存在する

最新のregionとzoneの数は以下で確認可能
ref: https://cloud.google.com/about/locations?hl=ja

## 環境への影響

Google Cloud使えば自動的に環境に配慮したものになるよーっていう自慢

## セキュリティ

リソース間の通信は、全てRPCであり暗号化されている