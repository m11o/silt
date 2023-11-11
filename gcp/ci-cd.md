# CI

cloud build

# CD

cloud deploy

# ソフトウェアサプライチェーンへの攻撃

ソフトウェアサプライチェーンってなんだっけ？

SolarWinds Orionプラットフォームに悪意あるコードを埋め込んだ

# CI/CDへの信頼

何を持って信頼があるとすると

- CI: 全てのテストに合格
- CD: これまでのプロセスを信頼

# CI/CDで考えること

## Secured Baseline

ネットワーク
宣言的インフラ(IaC)
クラスタ

階層的防御で分離する
[img]

## Source Code

脆弱性は73%がプログラミングエラー

## Artifact before deploy

デプロイする前に気をつけること
- docker image をslimに保つ
- マルチステージDockerビルド
- 不変のアーティファクトが必要
- Dockerfileのリンティング(hadolint)
- Distolessおよびマネージドベースイメージ

シークレットの管理
署名をしているもののみあげる