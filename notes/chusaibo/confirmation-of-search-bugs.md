https://adk-com.app.box.com/s/5pk3yfyb124pcm7zahxhuwlcklfbq9sx
https://adk-com.app.box.com/s/2x8bwxbytac9jtojru2se5s0eh8fh92a

# 詳細検索_共通エラー

## 商品が表示されない

出る
全部の商品が出てこないバグの修正前に検証したものかもしれない

## 並び順がリセットされる

Reactでデータを状態の管理(並び順や商品一覧)をしているので、ページをリロードするとデータがリセットされる。
browser backやforward でページを移動するときに、Stateを保持するようにする必要がある
-> そもそもrequest queryに持たせて、検索できるようにすればbrowser backなどでも検索結果が引き継がれるはず

なので対応方法はどちらか

1. 状態の遷移を遷移後も保持するようにする
2. リクエストパラメータに含めるようにする。
    - ブラウザバックのたびにAPIを叩く必要がある(しかも検索ワードにヒットする商品を全て取得する)

## 定期刊行物_著者名で検索できない

キーワードでは、以下の項目しか検索できない

- available_for_sale
- created_at
- product_type
- tag
- tag_not
- title
- updated_at
- variants.price
- vendor

詳細検索の著者名で検索できるているように見えてるが、実は全商品が出てきているだけ
絞り込みされているわけではない

例えば、著者名: 中央労働災害防止協会編で検索すると以下の商品が表示されるが、これの著者は中央労働災害防止協会編ではない
ref: http://localhost:3000/tosho/products/68003

対応方法

Search & Discoveryに `著者名等` を登録する
ただし、metafieldのタイプがmultiline textであるため、Search & Discoveryで登録できない。single line textに変更する必要がある

x -> authorName
◯ -> author-name

## 商品名に検索文字が一部含まれる商品が検索結果に含まれる

期待する挙動がわからない。
検索結果が含まれるのは良いのかなと思ったり、

完全一致だけを出したいってことかな？

## 判型での検索ができない

これもmetafieldがSearch & Discoveryに登録できないため
multiline textをsingle line textに変更する必要がある

## 商品名での検索ができない

バグっぽい

x -> productName
◯ -> product-name

ref: app/lib/products/search_common_loader.ts:67

## 品番での検索ができない

バグっぽい

そもそもform内のfield名がauthorNameになっている
productNumberに変更する

ref: app/lib/products/search_common_loader.ts:69

## カテゴリーで検索 No.20

これは難しい気がする。。。できなくはないが、複雑。。。
メガメニューと同じなので、カテゴリーの枠ごと消すでも良い気がする

## 作業内容

バグ

x -> jobDesc
◯ -> job-desc

Search & Discoveryに登録する

## 年のみで検索

年のみの検索はできない仕様になってそう

## 月のみの検索

月のみの検索はできない仕様になってそう

## maxYearでの検索

バグ
maxYearに値を渡さなければならないのに、minYearに値を渡している

## min&maxでの発行日検索

バグ
maxYearに値を渡さなければならないのに、minYearに値を渡している

## デジタルコンテンツ_制作発行年月日での検索

これはできない気がする
Search & Discoveryに登録すればできるが、metafieldを参照するので `20231130` みたいな形式で検索しないとヒットしない。
これをエンドユーザーに入力させるのは難しい

発行年月日での絞り込みができるので、そのバグを直して、そこで検索してもらう方が良さそう

## デジタルコンテンツ_ポスターが表示された

[デジタル版]と記述されているので、あってそうな気がするけど違うのかな？

## ISBNでの検索

metafieldへの登録が必要
multi line textをsingle line textに変更する必要がある

## 記念品_中災防で検索すると、マッチしない商品が出てそう

販売元に中災防という文言が入っており、かつvendorはShopifyのdefaultの検索対象なのであってそう
これはどうしようもない

## 記念品_サイズでの検索

Search & Discoveryに登録すればできるが、ユーザーが検索できるとは思えないので気にしなくても良い気がしてます

## 記念品_素材での検索

Search & Discoveryに登録すればできる
multi line textをsingle line textに変更する必要がある

## 記念品_仕様での検索

Search & Discoveryに登録すればできるが、これは内容的に複数行になる想定な気がするので登録できない気もする
multi line textをsingle line textに変更しても問題ないなら可能

## 記念品_社名印刷での検索

商品名にあるなら出ちゃうのは仕方ない気がする
checkboxなどで絞り込みを追加することも可能な気はするが、Search & Discoveryに登録してできるかは今の所不明

## 

