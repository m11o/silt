# ランダムな文字列を返す方法

math/randを使う

```go
import (
    "math/rand"
    "time"
)

func (ar ArticleRepository) GenerateHandle() string {
    source := rand.NewSource(time.Now().UnixNano())
    randInstance := rand.New(source)

    letters := []rune("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")
    b := make([]rune, 16) // 作成したい文字列の数を指定
    for i := range b {
        b[i] = letters[randInstance.Intn(len(letters))]
    }
    return string(b)
}
```
