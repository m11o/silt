# Requirements

1. Java JRE 11 以上
2. Google Cloud CLI
3. Google Cloud Client libraryを使用したアプリケーション

# Java JREのダウロード

以下URLから該当のplatformのJREをダウンロードする
https://www.oracle.com/java/technologies/downloads/#jdk21-mac

# Google Cloud CLIのインストール

1. Google Cloud 側にProjectを作成する
2. Pythonをインストールする (3.8以上)
3. gcloud CLIのパッケージをインストールする
    - https://cloud.google.com/sdk/docs/install-sdk?hl=ja
4. 以下コマンドを実行して、gcloud CLIを初期化する
    - ```shell
      $ ./google-cloud-sdk/install.sh
      $ ./google-cloud-sdk/bin/gcloud init
      ```
5. ターミナルを再起動

ref: https://cloud.google.com/sdk/docs/install-sdk?hl=ja
      
# エミュレータの起動

```shell
gcloud beta emulators firestore start

# hostとportを指定する場合

gcloud beta emulators firestore start --host-port=localhost:8043
```
