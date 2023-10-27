# simple rails

## RDB

アプリケーションをシンプルに実装するTIPS
シンプルさ => サービスでやりたいことと、記述するコードの差が少ない状態

scaffoldがヒントになる
普通のサービスはCRUDではない？ => それはそう

ドメインに現れるイベントエンティティを考える
=> resource entity => モノを表す
-> event entity => コトを表す

エンティティを独立として扱いたい場合は、has_many :through
-> rails tutorialに記述される

中間テーブルをイベントエンティティとして考えると良い
後続イベントの親レコードになる
付加情報をつけることもできる

見つけることが難しい

↑これ自体は、texta.fmでも同じこと言ってた

-> 利用ポイントを保存することは、コトをリソースとして使用できるかも

## REST

routingはresource と resourcesを使用
リソースをHTTP経由でCRUDする

コトをnewするとは？ => 不要ならいらない

### その他

session#new, session#create, session#destroyはセッションをCRUDする
ものやこと以外をCRUDしている

## Ruby

strong parameter

ActiveModelを使って、I/Fに近い別レイヤーを導入する

resourceの粒度とレコードの粒度は違う
-> FormObjectを追加するべき

Kaigi on Railsは現在の話が多い気がする
-> 未来の話を聞きたいなー
