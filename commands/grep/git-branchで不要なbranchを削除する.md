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