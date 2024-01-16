# golangの継承

そもそも継承はない。
オブジェクト指向的に書くなら、継承。
どちらかというと委譲。

## 書き方

```go
type User struct {
    ID   int    `json:"id"`
    Name string `json:"name"`
}

func (u *User) hello() string {
    return "hello"
}

type Admin struct {
    User
    Role string `json:"role"`
}

func main() {
    a := Admin{
        User: User{
            ID:   1,
            Name: "foo",
        },
        Role: "admin",
    }
	// Userのメソッドを直接呼び出せる
	// admin.user.hello()という風に呼び出す必要はない
    fmt.Println(a.hello()) // hello
	fmt.Println(a.ID) // 1
    fmt.Println(a) // {{1 foo} admin}
}
```

ref: https://www.geeksforgeeks.org/inheritance-in-golang/

## パターン

- 構造体に構造体を埋め込む
- インターフェースにインタフェースを埋め込む
- 構造体にインターフェースを埋め込む
  - 構造体によるインターフェースの実装も当てはまるみたい。
- 委譲のネスト

ref: https://qiita.com/gold-kou/items/514aeeba16b59284bbf6
