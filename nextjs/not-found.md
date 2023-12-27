# notFoundの扱い方

next.js 13以降では、notFound()メソッドを使用することによって、 `NEXT_NOT_FOUND` が発生させることができ、404ページを表示することができる。

これは、データ取得時に投げるのではなく、ページの初期化時に投げる必要がある。

```tsx
import { notFound } from 'next/navigation'
 
async function fetchUser(id) {
  const res = await fetch('https://...')
  if (!res.ok) return undefined
  return res.json()
}
 
export default async function Profile({ params }) {
  const user = await fetchUser(params.id)
 
  if (!user) {
    notFound()
  }
 
  // ...
}
```