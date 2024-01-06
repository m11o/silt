# スライスから要素を削除する方法

## 要素の順番を無視していい場合

```go
slice1 = []int{1, 2, 3, 4, 5}
slice1[2] = slice1[len(slice1)-1] // 最後の要素を削除したい要素に代入
slice1 = slice1[:len(slice1)-1]   // 最後の要素を削除
```

## 要素の順番を無視しない場合

## copyを使う

O(n)の計算量

```go
a := []string{"A", "B", "C", "D", "E"}
i := 2

copy(a[i:], a[i+1:]) // Shift a[i+1:] left one index.
a[len(a)-1] = ""     // Erase last element (write zero value).
a = a[:len(a)-1]     // Truncate slice.
```

## appendを使う方法

O(n)の計算量

```go
slice1 = []int{1, 2, 3, 4, 5}
slice1 = append(slice1[:2], slice1[3:]...)
```
