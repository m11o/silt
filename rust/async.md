# mainで非同期処理を実行する方法　

async-stdを使用する
ref: https://docs.rs/async-std/latest/async_std/

tokioも有名だが、非同期通信だけできるようにしたいならasync-stdの方が良さそう。
細かい違いについてはちゃんと調べていない

また、インストールするときにfeaturesに `attributes` を指定する必要がある。defaultではダウンロードに時間がかかるため、defaultでは無効になっている

## TODO

- async-stdとtokioの違いを調べる