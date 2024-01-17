mysqlではデフォルトで、case insensitiveになっており、大文字・小文字を区別しない
確認方法は、collationを確認する必要がある。 **_ci(ci: case_insensitiveの略)かどうかを確認すれば良い

以下、コマンドでテーブルの全カラムの情報を確認することができる

```sql
SHOW FULL COLUMNS FROM agents;
```