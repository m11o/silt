# context.Context型の役割

ref: https://zenn.dev/hsaki/books/golang-context/viewer/definition

## ざっくり概要

go-routine間で値を共有するための仕組み
一つの処理が複数のgo-routineで実行される場合に、値を共有することができる

- 処理の締め切りを伝達
- キャンセル信号の伝播
- リクエストスコープ値の伝達

## そもそもgo-routineの情報伝達方法

goにおいては、並列処理のデータのやり取りは、チャネルを使って行うことを推奨している
ロックをとって、メモリを共有することによってデータをやり取りする一般的な方法は非推奨

## context.Context型の定義

```go
type Context interface {
    Deadline() (deadline time.Time, ok bool)
    Done() <-chan struct{}
    Err() error
    Value(key interface{}) interface{}
}
```

4つのメソッドを使って、異なるgo-routineの間の情報を受け取ることができる

## context.Background()

context.Background()は、空のContextを返す
「キャンセルされない」「deadlineも持たない」「共有する値も何も持たない」状態のcontextが返される

```go
context.Background()
```

## Done()メソッド

キャンセルした場合に、キャンセルされたことを伝えるためのチャネルを返す

```go
var wg sync.WaitGroup

func generator(ctx context.Context, num int) <-chan int {
	out := make(chan int)
	go func() {
		defer wg.Done()

	LOOP:
		for {
			select {
            case <-ctx.Done(): // キャンセルを受け取った場合に分岐
				break LOOP
			case out <- num:
			}
		}

		close(out)
		fmt.Println("generator closed")
	}()
	return out
}

func main() {
	ctx, cancel := context.WithCancel(context.Background())
	gen := generator(ctx, 1)

	wg.Add(1)

	for i := 0; i < 5; i++ {
		fmt.Println(<-gen)
	}
	cancel() // キャンセルを伝える

	wg.Wait()
}
```