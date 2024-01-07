# ポインタの扱い方

## 変数

メモリアドレスを参照

```go
import "fmt"

num := 1
fmt.Println(num)  // 1
fmt.Println(&num) // 0xc0000b4008
```

メモリを保持する変数を定義

```go
num := 1
var a *int = &num
```

ポインタが指し示すアドレスに保持されている値を取得する

```go
name := "John"
var ptr *string

ptr = &name

// * to get the value pointed by ptr
fmt.Println(*ptr) // John
```

## 関数

関数の引数にポインタを渡す

```go
func change(val *int) {
    *val = 55
}

func main() {
    val := 10
    fmt.Println("value before function call is:", val) // 10
	
    chnage(&val)
    fmt.Println("value after function call is:", val) // 55
}
```

関数の戻り値にポインタを返す

```go
func hello() *int {
    i := 5
    return &i
}

func main() {
	var i *int = hello()
    fmt.Println("value of i:", *i) // 5
	fmt.Println("address of i:", i) // 0xc0000b4008
}
```

# References

- https://www.programiz.com/golang/pointers
- https://www.programiz.com/golang/pointers-functions
