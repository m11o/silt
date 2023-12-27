# use clientの使い所

next.js 13以降では、server sideレンダリングがデフォルトになった。
また、useStateなどのクライアントサイドの処理は、page表示するコンポーネントとはファイルを分けて、`use client` というキーワードを表示する必要がある

```typescript jsx
/* counter.tsx */
'use client'

import { useState } from 'react'

export default function Counter({ defaultValue }: { defaultValue?: number }) {
  const [count, setCount] = useState(defaultValue || 0)

  return (
    <div>
      <p>count: {count}</p>
      <button onClick={() => setCount(count + 1)}>+1</button>
    </div>
  )
}

/* page.tsx */
import Counter from './counter'

export default function Page() {
  return (
    <div>
      <Counter defaultValue={2} />
    </div>
  )
}
```
