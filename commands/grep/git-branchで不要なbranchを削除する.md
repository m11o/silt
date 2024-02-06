# 不要なbranchの削除するワンライナーコマンド

```shell
git branch | grep -v "^[\s\*]{2}(master|main|develop)$" | xargs git branch -d
```

1. git branchでブランチ一覧を取得
2. grepでmaster, main, developを除外した全てのbranchを取得
   - ただし、git-branchコマンドでは先頭に半角スペースか `*` が付与されているため、それを除外するために正規表現で `[\s\*]{2}` を指定
   - -vは除外するオプション
3. xargsで取得したbranchをgit branch -dで削除
   - -dでブランチを削除するオプション
   - 強制的に削除したいなら `-D` オプション

## formatを指定

formatを指定すれば、無駄な正規表現を書く必要なく、branch名を取得できる

```shell
git branch --format="%(refname:short)" | grep -vE "^(master|main|develop)$" | xargs git branch -d
```

1. git branch --format="%(refname:short)"でbranch名を取得
   - formatオプションで出力するformatを指定することができる
2. ...残りは同じ

formatの指定方法はgit-for-each-refコマンドを参照
ref: https://git-scm.com/docs/git-for-each-ref
