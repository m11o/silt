# mode

"cors", "navigate", "no-cors", "same-origin", "websocket" の5つがあるっぽい
以下にそれぞれ解説

https://fetch.spec.whatwg.org/#concept-request-mode

no-corsだとgetとpostはできるけど、deleteはできないので注意
許容されているメソッドは `GET` `POST` `HEAD` のみ
ref: https://fetch.spec.whatwg.org/#cors-safelisted-method

no-corsだと、まさかのAuthorization headerも送信できないのか
ref: https://fetch.spec.whatwg.org/#cors-safelisted-request-header