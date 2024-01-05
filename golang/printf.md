# fmt

3つのメソッドがある

- fmt.Print
- fmt.Println
- fmt.Printf

## fmt.Print

カンマ区切りで複数の文字列を結合して、出力することができる。ただし、引数ごとにspaceが入らないので、スペースを入れたい場合には明示的にスペースを入れる必要がある。
また、最後に改行が入らないので、 改行する場合には改行コードが必要

```go
a, b := 3.0, 4.0
h := math.Hypot(a, b)

// 引数と引数の間にspaceは入らないので、明示的に追加する必要がある
fmt.Print("The vector (", a, b, ") has length ", h, ".\n")
```

## fmt.Println

最後に必ず改行が入る。Printと同じくカンマ区切りで複数の文字列を結合して、出力することができる
ただし、引数同士でspaceが入るため、明示的にspaceを追加する必要はない

```go
a, b := 3.0, 4.0
h := math.Hypot(a, b)

// 明示的にspaceが入るので、spaceは不要
fmt.Println("The vector (", a, b, ") has length", h, ".")
```

## fmt.Printf

C言語のprintfと同じように、フォーマットを指定して出力することができる。
最終行に改行コードが必要。


```go
a, b := 3.0, 4.0
h := math.Hypot(a, b)

fmt.Printf("The vector (%g %g) has length %g.\n", a, b, h)
```

### 使用可能なフォーマット

| フォーマット | 説明 |
| --- |  |
| %d | 10進数 |
| %x, %o, %b | 16進数, 8進数, 2進数 |
| %f, %g, %e | 浮動小数点数 |
| %t | 真偽値 |
| %v | 任意の値 |
| %T | 値の型 |
| %s | 文字列 |
| %q | a double-quoted string safely escaped with Go syntax |
| %x, %X | base 16, lower-case or upper case, two characters per byte |
| %p | ポインタの値, slice |
