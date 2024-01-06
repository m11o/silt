# net/httpにおけるJSONのレンダリング

(chiでも同じ)

encoding/jsonに含まれる `json.Marshal` を使う。
ref: https://pkg.go.dev/encoding/json

```go
import (
    "encoding/json"
    "net/http"

    "github.com/go-chi/chi"
)
type User struct {
    ID   int    `json:"id"`
    Name string `json:"name"`
}

func main() {
    r := chi.NewRouter()
    r.Get("/", func(w http.ResponseWriter, r *http.Request) {
        u := User{ID: 1, Name: "foo"}
        w.Header().Set("Content-Type", "application/json")
		json.Marshal(u)
    })
    http.ListenAndServe(":8080", r)
}
```

Encoderを使用することも可能。
この場合、io.Writerの形で自由に渡すことができる。
例えば、シリアライズ結果をファイルに書き込むことも可能。

```go:main.go
import (
    "encoding/json"
)

type User struct {
    ID   int    `json:"id"`
    Name string `json:"name"`
}

func main() {
    u := User{ID: 1, Name: "foo"}
    f, _ := os.Create("user.json")
    defer f.Close()
    json.NewEncoder(f).Encode(u) // user.jsonに書き込まれる
}
```

## references

- https://zenn.dev/hsaki/articles/go-convert-json-struct
