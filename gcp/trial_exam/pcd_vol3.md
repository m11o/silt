# subnet範囲の拡張

拡張自体はできる
ロードバランサ プロキシ専用に使用されるサブネットは拡張出来ない
サブネットのプライマリ IPv4 範囲の拡張は、完了までに数分かかる
拡張中、サブネット内のトラフィックは中断されない

# firewall priority

priorityは0から65535までの整数
priorityが小さいほど優先度が高い

# Serverless VPC Access

Cloud Run や Cloud Functions, App Engine から VPC 内のリソースにアクセスするための機能

# Compute Engine の永続ディスク

永続ディスクはいつでも容量をあげることができる
ただし、サイズを小さくすることができない。小さくする場合には、新しいディスクを作成する必要がある

# Cloud KMS

Key Management Service
鍵を管理して、暗号化情報などに使用することができる

# IPv6を使用できるロードバランサー

外部HTTP(S)ロードバランサー
SSLプロキシロードバランサー
TCPプロキシロードバランサー
