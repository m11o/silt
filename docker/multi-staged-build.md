# マルチステージビルド

FROMを複数回使用して、imageを最小限にすることができる
本番環境などで、不要なライブラリなどを含めないようにすることができる

```Dockerfile
FROM ruby:3.1.1-slim-buster AS base
WORKDIR /app

RUN apt-get install -y build-essential
COPY Gemfile Gemfile.lock ./

RUN bundle install --deployment

# これ以下だけのimageが作成される
FROM base

COPY . $RAILS_ROOT
COPY --from=base /var/www/app/Gemfile.lock "${RAILS_ROOT}"
EXPOSE 3000
```

以前までは

> 開発用にはアプリケーションのビルドに必要な全てが含まれるDockerfileを使用し、プロダクト用にはアプリケーションおよび実行に必要なもののみが含まれるスリム化されたDockerfileを使用するという事が行われ、これが非常に一般的だった。
> これがいわゆるビルダーパターン(開発パターン)と呼ばれるもの。

ref: https://matsuand.github.io/docs.docker.jp.onthefly/develop/develop-images/multistage-build/
ref: https://zenn.dev/masaruxstudy/articles/d85f6c1af3bf65
