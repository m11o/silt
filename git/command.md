# git-switch

git-checkoutの代わりにbranchを切り替えることができる
`-c` or `-C` を使用した場合には、新しいbranchを作成した上で切り替えることができる
ただし、-Cはbranchがすでに存在した場合も強制的にbranchを作成するため、注意が必要

`-` オプションを使用すると、直前のbranchに切り替えることができる

**ただし、このコマンド自体はまだ実験的な機能なので新しいバージョンでは変更がある可能性あり**

## なぜgit-checkoutではなくgit-switchが作成されたのか

git-checkout ブランチ切り替えだけでなく、コマンドは変更の取消機能があり、分かりずらい仕様になっていた
git-switch はブランチ切り替えのみを行うコマンドとして作成された

## references

- https://git-scm.com/docs/git-switch
- https://tracpath.com/docs/git-switch/
- https://qiita.com/RINYU_DRVO/items/958e0723178f88da8101
- https://snowsystem.net/git/git-command/git-switch/

# git-restore

git-checkoutの代わりにファイルを復元することができる
`--staged` or `-S` が指定された場合には、HEADから復元される。それ以外の場合にはindexから復元される
別のcommitから復元するためには、 `--source` or `-s` を使用する

**ただし、このコマンド自体はまだ実験的な機能なので新しいバージョンでは変更がある可能性あり**

## references

- https://tracpath.com/docs/git-restore/
- https://www.r-staffing.co.jp/engineer/entry/20200131_1
