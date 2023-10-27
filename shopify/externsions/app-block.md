# Theme extension

Theme extension のApp blockを使用すれば、テーマ内にblockを追加できる
ref: https://shopify.dev/docs/apps/online-store/theme-app-extensions/extensions-framework

embed app blockは、bodyやheadにblockを直接埋めるこtができる

それぞれ、
app block: target == section
embed app block: target == body or head

# settings

おそらくこれが正しいURL: https://shopify.dev/docs/apps/online-store/theme-app-extensions/extensions-framework#schema

## enabled_on 

ref: https://shopify.dev/docs/themes/architecture/sections/section-schema#enabled_on

利用可能なテンプレートを指定できる
templateの種類は以下を参照
https://shopify.dev/docs/api/liquid/objects/request#request-page_type

## limit

一つのテンプレートに表示できる個数
ref: https://shopify.dev/docs/themes/architecture/sections/section-schema#limit

## input setting

Shopify管理画面から設定可能な項目を定義する
https://shopify.dev/docs/themes/architecture/settings/input-settings

# javascript

{% javascript %} と <script>の違い
{% javascript %}内ではliquidのコードを使用することができない。<script>内ではliquidのコードを使用することができる。

{% javascript %}内のコードはDOMの構築完了後に読み込まれる
おそらくstylesheetも同じ
