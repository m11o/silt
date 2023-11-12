# github actionsでmysqlに繋げる

## はまったポイント

- github actionsが用意しているservice containerは127.0.0.1かlocalhostで接続可能
- mysqlの起動時にhealth checkを追加する必要がある
  - 以下のエラーが発生する
  - ```shell
    Lost connection to MySQL server at 'reading initial communication packet', system error: 0

    Caused by:
    Mysql2::Error::ConnectionError: Lost connection to MySQL server at 'reading initial communication packet', system error: 0 (Mysql2::Error::ConnectionError)
    ```
- github actions の場合にはroot userにして、passwordを設定してはいけない(かも)
- DBのcharsetなどをgithub actionsの場合には変更できない
  - ```
    [
      --character-set-server=utf8,
      --collation-server=utf8_general_ci,
      --default-storage-engine=innodb,
    ]
    ```

service containerについては以下参照
ref: https://docs.github.com/ja/actions/using-containerized-services/about-service-containers

## 最終的なもの

```yaml
name: "CI"
on:
  push:
    branches: [ "main" ]
  pull_request:
    branches: [ "main" ]
jobs:
  test:
    runs-on: ubuntu-latest
    services:
      mysql:
        image: mysql:8.0
        ports:
          - "3306:3306"
        env:
          MYSQL_DATABASE: portfolio_api_rails_test
          MYSQL_ALLOW_EMPTY_PASSWORD: 1
        options: --health-cmd "mysqladmin ping" --health-interval 20s --health-timeout 10s --health-retries 10 # ここにhealth checkを入れないとlost connectしてしまう
    env:
      RAILS_ENV: test
    steps:
      - name: Checkout code
        uses: actions/checkout@v3
      - name: Install Ruby and gems
        uses: ruby/setup-ruby@55283cc23133118229fd3f97f9336ee23a179fcf # v1.146.0
        with:
          bundler-cache: true
      - name: Set up database schema
        run: bundle exec rails db:schema:load
      - name: Run tests
        run: bundle exec rspec

  lint:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v3
      - name: Install Ruby and gems
        uses: ruby/setup-ruby@55283cc23133118229fd3f97f9336ee23a179fcf # v1.146.0
        with:
          bundler-cache: true
      - name: Lint Ruby files
        run: bundle exec rubocop --parallel

```