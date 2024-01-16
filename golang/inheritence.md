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