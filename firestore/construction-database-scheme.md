ref: https://gist.github.com/matope/2396234

# NoSQLのデータモデリングの考え方

自分は何を知りたいか

## 非正規化

- クエリをシンプルにできる
- データをコピーして保存することもある

### pros/cons

- 「クエリデータボリュームまたはクエリごとのIO」 VS 「トータルのデータボリューム」
- 「処理の複雑性」 VS 「トータルのデータボリューム」

## 集計

RDBMSだと正規化しないと表現できないデータ構造(one-to-many, many-to-many)を、NoSQLでは集計によって表現する。

```json
{
  "type": "Book",
  "id": 1234567890,
  "price": 1000,
  "description": "This is a book about John Doe.",
  "details": {
    "title": "John Doe",
    "author": "John Doe"
  }
}
```

## Join

NoSQLの場合には、joinは設計時に行うことが基本。
ただし、編集・更新が多く入るデータの場合には、クエリの実行時にjoinをした方がよい