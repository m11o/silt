# WWW-authenticate

ref: https://developer.mozilla.org/ja/docs/Web/HTTP/Headers/WWW-Authenticate

リソースへのアクセス権を得るための認証方式メソッドを定めることができる。

## フロー

1. リクエストを送信
2. サーバーから401レスポンスと、1つ以上のチャンレンジ(認証方式)をもつWWW-Authenticateを受け取る
3. クライアントページにユーザー情報を入力欄を表示する
4. ユーザーが入力した情報をサーバーに送信する
5. サーバーはユーザー情報を検証する
6. サーバーはユーザー情報が正しければ、リクエストを処理する
7. サーバーはレスポンスを返す

## format

```
WWW-Authenticate: <auth-type1>, <auth-type2>, ...<auth-typeN>

or

WWW-Authenticate: <auth-type1>
WWW-Authenticate: <auth-type2>
...
WWW-Authenticate: <auth-typeN>
```

## auth-type(チャレンジ、認証方式)

有名なものに `Basic` `Digest` `Nagotiate` `AWS4-HMAC-SHA256` などがある(大小区別なし)

### Basic

受け取れすDirectiveは以下の通り

- realm
- charset

WWW-Authenticate ヘッダーを含むレスポンスは以下のようなもの

```http request
WWW-Authenticate: Basic realm="Access to the staging site", charset="UTF-8"
```

このヘッダーを受け取ったブラウザ(エージェント)は、ユーザー名とパスワードの入力欄を求める
入力されたユーザー名とパスワードを以下のフォーマットでサーバー側に送信する

```http request
Authorization: Basic YWxhZGRpbjpvcGVuc2VzYW1l
```

Basic認証では、資格情報のユーザー名とパスワードをコロン(:)で繋げて、base64でエンコードしたものを送信する

