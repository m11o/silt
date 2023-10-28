# CQRS

更新と参照する責務を分離する

更新 = command
参照 = query

Client -> useCase -> command -> model

model -> query -> client

サービスクラスとの違いは?

commandではデータの整合性のvalidationを書いている
use_caseでは認可のvalidation(?)を書いている

更新と参照を両方行うことがあるがどうするのか?

youtrustのopencodeというイベントがあるみたい