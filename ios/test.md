# Snapshot Test

変更前の出力結果と変更後の出力結果を比較し、差分がないことをテストする

## ライブラリ

### iOSの場合

https://github.com/uber/ios-snapshot-test-case
https://github.com/pointfreeco/swift-snapshot-testing

## references

https://lab.mo-t.com/blog/ios-snapshot-test

## 疑問

1. UI test と snapshot test の違いは？

UI testはコードベースで記述するテスト。
snapshot testは画像を比較するテスト。詳細はライブラリを読む必要があるが、binary等で比較していると考えられる

2. 明示的に変更があった場合には、どのように正であると判断するのか?

新しいデザインを登録すると、新しいsnapshotを登録する

