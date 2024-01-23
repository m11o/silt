# Dockerでfirestore emulatorを起動する方法

googleがdocker imageを提供しているので、それを使う

ref:  gcr.io/google.com/cloudsdktool/google-cloud-cli:emulators

`:emulators` はエミュレータ周りのツールが入っているイメージ。
他のlabelは以下の通り

- `:latest` : 追加コンポーネントがプリインストールされている大規模（Debian ベース）イメージ
- `:slim` : 追加コンポーネントがプリインストールされていない小規模（Debian ベース）イメージ
- `:alpine` : 追加コンポーネントがプリインストールされていない小規模（Alpine ベース）イメージ
- `:debian_component_based` : Debian ベースのすべてのコンポーネント。
  - latestとは異なり、コンポーネントマネージャーを使用して、必要なコンポーネントがインストールされている 

ref: https://cloud.google.com/sdk/docs/downloads-docker?hl=ja

VolumeやPORTを設定する場合にはDockerfileを作成するか、docker-compose.ymlに明記する
