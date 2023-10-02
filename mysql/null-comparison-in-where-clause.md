# NULLの比較

is nullを使う。

null != 1やnull <> 1はnullを返す
null = nullもnull

!=, =, <> などの算術演算子は、nullでは使用できない

なので、 

```sql
select * from table where column is null; # ok
select * from table where column not in ('hoge'); # これはcolumnにnullは含まれない
```

not inは<>を配列で実行しているので、null != hogeはnullを返し、nullのレコードは返さない
