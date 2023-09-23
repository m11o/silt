# App Engine

IntelJ IDEAやEclipseなどの開発環境からアプリを作成できる
様々なサービスとAPIが組み込まれている
- memcached
- nosql
- health check

ローカルマシン上で開発やデプロイなどを管理することができる
セキュリティは、security command centerで自動で脆弱性をスキャン・検知できる

GKEとApp Engineの違い
App Engine standardは、デプロイやスケーリングをできるだけ自動化したい人向け
GKEは、より柔軟な設定ができる
App Engine FlexibleはApp Engine standardとGKEの中間

App Engine standard 
VMインスタンスは、regionごとに自動的に配布される
localで開発-> デプロイ -> デプロイされたコードが実行されるVMインスタンスにアクセスする

Flexibleの方は、コードに集中しながらある程度、ランタイムやOSをカスタマイズできる(Dockerfileに記述)

# Cloud API 管理

バージョニングを行うことができるので、昨日の追加や削除を行うことができ、APIにリクエストする場合には、バージョンを指定する必要がある

cloud endpoint
hosting, loging, API 作成, monitoringを行うことができる
Open API準拠

Appiee edge
レート制限、割り当て、分析のような ビジネス上の問題に特に重点を置く

# Cloud run

knaitiveで実装されている
ref: https://cloud.google.com/knative?hl=ja

リクエストした部分とshutdown, start時点しか課金されない
リクエストを処理しない限りは課金されない
compute engineは起動している間は課金される

ソースコードベースでもコンテナベースのものでもデプロイできる
ソースコードベースの場合には、buildpacksというライブラリが使用されている


Artifact Registryからイメージをpullできる


