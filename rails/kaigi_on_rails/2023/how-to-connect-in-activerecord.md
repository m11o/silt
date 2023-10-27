見つけた方、ブレイクポイントを仕込んで、SQLを実行した時のbacktraceを確認する
-> rdbg

# activerecord

## motivattion

activerecordのことを知ることによって、アプリケーションごとに

## 仕組み

### クライアントとサーバーの接続

- tcpプロトコル
- mysql接続
  - mysqlプロトコル

server gating
login

Mysql2Adapterのインスタンスごとに接続している

### クライアント側とサーバー側の処理

アダプタを使用してmysqlの接続を確立(mysql2, libmysqlclient)
ARはmysql2のインスタンを作成することで、接続を待つ

AR -> mysql2(libmysqlclient) -> DB(mysql)

### ARの接続までのクラスとモジュール

#### 抽出

ブレイクポイントを張って、rdbgでステップ実行

#### Mysql2Adapter

AbstractMysqlAdapter(抽象クラス)
mysql2を用いたDB接続を行う

#### ConnectionHandle

ConnectionPoolを管理する
データベースごとに接続プールを保持している
どのConnectionPoolを使用するかみる

#### ConnectionPool

### ARに接続の見方

Mysql2::Client.new(config)
=> configにはuserやpw
=> initializeメソッドでlibmysqlclientによるmysql接続が行われる


database.ymlから作成したいアダプタを取得
base.public_send()
check_versionでdbのバージョンとサーバーへの接続要求を行う

#### connection_poolを取り出す

ConnectionPool#checkout
-> aquireconnection => 接続queueを確認

#### connection_poolを作成

establish_conectionで作成
owner_name => ApplicationRecord, WorldRecord みたいなやつ

pool_managerはowner_nameごとに一つ作成

role(writer, reader)ごとにPoolConfigを作成
poolConfig -> ConnectionPool

connection_handler -> pool_manager -> pool_config -> connection_pool

## エラー原因の切り分け方法

ConnectionNotEstablished
new_clientの前でエラーなら、接続前の段階
  -> 設定や待ち時間など
new_clientの後のエラーなら、接続ごの段階

## DBホスト変更の注意点

failoverした場合は、切り替え前のものに接続される可能性

# appendix

## sharding

テーブルを小さなパートに分けて実装
