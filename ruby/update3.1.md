# 3.0 -> 3.1の変更

メソッドの追加や新機能などは除外しています

- 多重代入の評価順序が単一代入の評価順序と同じになった
- Dir.globにFile::FNM_DOTMATCHオプションを渡したときに `..` が返らなくなった
- each_consとeach_sliceが戻り値としてレシーバを返す
- private/protected/public/module_functionが引数を返す
- keyword_initオプションを明示的に指定せずに生成されたStructに対してキーワード引数だけでnewしようとすると警告
- $LOAD_PATH.resolve_feature_pathにloadできない値を渡しても例外が発生しなくなった
- refine内でのinclude/prependが非推奨
- 一部のdefault gemがbundled gem
  - net-ftp 0.1.3
  - net-imap 0.2.2
  - net-pop 0.1.1
  - net-smtp 0.3.1
  - matrix 0.4.2
  - prime 0.1.2
  - debug 1.4.0
- ERB#initialize の第2引数以降が常に警告

ref: https://qiita.com/jnchito/items/bcd9b7f59bf4b30ea5b3
ref: https://www.ruby-lang.org/ja/news/2021/12/25/ruby-3-1-0-released/
ref: https://github.com/ruby/ruby/blob/v3_1_0/NEWS.md
ref: https://techlife.cookpad.com/entry/2021/12/25/220002

## 使用箇所確認

### 多重代入の評価順序が単一代入の評価順序と同じになった

多重代入の箇所は何ヶ所かあったが、特に問題なさそう

```shell
$ grep -rnE '^\s*([a-zA-Z_][a-zA-Z_0-9]*\s*,\s*)+[a-zA-Z_][a-zA-Z_0-9]*\s*=\s*.+' --exclude-dir=logs --exclude-dir=node_modules --exclude-dir=db --exclude-dir=public --exclude-dir=vendor .
        
./webhook_endpoint/app/models/webhook_endpoint/stores/webhook_parameters.rb:32:        payload, _header = JWT.decode(jwt, nil, true, { algorithms: jws_algorithms, jwks: jwks })
./app/middleware/rack/biz_client.rb:23:      status, headers, body = @app.call(env)
./app/models/shop_message.rb:805:    sent, pushed, confirmed, visited = messages.pick(Arel.sql('SUM(sent_at is not null) count, SUM(pushed_at is not null) count, SUM(confirmed_at is not null) count, SUM(visited_at is not null) count'))
./app/models/target/shop_card_rank.rb:42:    rank_seq_from, rank_seq_to = ShopCardRank.where(
./app/models/stores/retail/item/fetch_published_items_for_home.rb:18:            min_price, max_price = minmax_price_from_variations(item[:variations])
./app/models/util.rb:55:      y1, x1, y2, x2 = [*pos1, *pos2].map { |v| v * Math::PI / 180 }
./app/models/user.rb:145:      email, password = attr.values_at(:email, :password)
./app/models/user.rb:381:      checkin_result, returned_desc = shop_barcode.validate_code(self, values)
./app/lib/fcm.rb:86:    resp, dat = http.post(url.path, data, headers)
./app/lib/stores/retail/store_session_creator.rb:29:        session_id, expires_at = CGI::Cookie.parse(session_cookie).slice(RETAIL_SESSION_KEY, 'expires').values.flatten
./app/controllers/app/user/user_cards_controller.rb:22:    lat, log, distance = Util.get_default_location(_lat, _log)
./app/controllers/app/user/shop_branches_controller.rb:10:    lat, log, distance = Util.get_default_location(_lat, _log)
./app/controllers/shops/user_cards_controller.rb:8:    result, msg = @user_card.recover_data
./app/controllers/shops/users_controller.rb:105:      shop_card_id, shop_branch_id = params[:shop_card_and_branch_id].split('_')
./app/controllers/api/app/v1/shop_branches_controller.rb:20:    lat, log, distance = Util.get_default_location(_lat, _log)
./app/controllers/api/app/v2/tmpl/mini_app_items_controller.rb:128:    lat, log, distance = Util.get_default_location(_lat, _log)
./app/controllers/api/app/v2/tmpl/shop_branches_controller.rb:12:    lat, log, distance = Util.get_default_location(_lat, _log)
./app/controllers/api/app/v2/tmpl/users_controller.rb:55:    user, user_uuid, access_token = User.register!(
./app/controllers/api/app/v2/tmpl/stores_ec/users_controller.rb:103:    user_uuid, access_token = nil
./app/controllers/api/app/v2/mini_app_items_controller.rb:10:    lat, log, distance = Util.get_default_location(_lat, _log)
./app/controllers/api/app/v2/shop_branches_controller.rb:14:    lat, log, distance = Util.get_default_location(_lat, _log)
./app/controllers/api/app/v2/users_controller.rb:17:    user, user_uuid, access_token = User.register!(
./app/controllers/api/app/v2/topics/shop_branches_controller.rb:8:    lat, log, distance = Util.get_default_location(_lat, _log)
./app/controllers/api/app/v2/topics/pickups_controller.rb:8:    lat, log, distance = Util.get_default_location(_lat, _log)
./app/controllers/api/app/v2/shop_coupons_controller.rb:8:    lat, log, distance = Util.get_default_location(_lat, _log)
./app/controllers/api/biz/v1/shop_barcodes_controller.rb:75:    user, result = User.find_or_create_by_ics_then_checkin(shop_barcode: @shop_barcode,
./spec/models/staff_id_token_spec.rb:34:        header, original_payload, signature = original.split('.')
./spec/requests/staffs/passwords_spec.rb:94:        raw, hashed = Devise.token_generator.generate(Staff, :reset_password_token)
./spec/requests/staffs/passwords_spec.rb:161:        raw, hashed = Devise.token_generator.generate(Staff, :reset_password_token)
./shopify/app/jobs/shopify/orders_fulfilled_job.rb:55:        product, variant = prepare_product_and_variant_records(line_item)
./shopify/app/controllers/shopify/social_plus/authentications_controller.rb:24:        social_plus_user_id, shopify_user_gid = social_plus_client
./shopify/app/controllers/shopify/social_plus/authentications_controller.rb:38:          user, uuid, access_token = create_user_login_data_for_default_login(ec_user).slice(:user, :uuid, :access_token).values
./shopify/app/controllers/shopify/social_plus/authentications_controller.rb:49:          user, uuid, access_token = create_user_login_data_and_create_ec_user(shopify_user_id).slice(:user, :uuid, :access_token).values
./shopify/app/controllers/shopify/users_controller.rb:31:      user, user_uuid, access_token = nil
./lib/tasks/oneshot/20230817_remove_infinite_ttl_key_for_session.rake:17:      cursor, keys = redis.scan(cursor, scan_options)
./lib/tasks/oneshot/20200908_observation_point_seed.rake:74:        lon, lat = Geo.geocode(address).values
./lib/tasks/database.rake:12:    status, _, stderr = systemu command
```

### Dir.globにFile::FNM_DOTMATCHオプションを渡したときに `..` が返らなくなった

使用箇所なし

### each_consとeach_sliceが戻り値としてレシーバを返す

#### each_cons

blockなしで使用しているため、Enumeratorが返っているので問題なし

```shell
$ grep -rn each_cons --exclude-dir=logs --exclude-dir=node_modules --exclude-dir=db --exclude-dir=public --exclude-dir=vendor .

./app/models/user_checkin.rb:619:        update_data = user_checkins.each_cons(2).each_with_object({}) do |(prev_c, next_c), result|
```

#### each_slice

blockありがほとんど
ただし、その返り値を使用して何かを行なっているわけではなさそう

```shell
$ grep -rn each_slice --exclude-dir=logs --exclude-dir=node_modules --exclude-dir=db --exclude-dir=public --exclude-dir=vendor --exclude-dir=tmp .

./app/models/export.rb:640:          shop_message_ids.each_slice(100) do |ids|
./app/models/export.rb:700:          used_coupon_ids.each_slice(100) do |ids|
./app/models/export.rb:759:          revoked_coupon_ids.each_slice(100) do |ids|
./app/models/export.rb:845:          user_ids.each_slice(1000) do |ids|
./app/models/export.rb:922:          user_campaign_ids.each_slice(100).each do |ids|
./app/models/event.rb:26:      pluck(:doc_id).each_slice(1024) do |ids|
./app/lib/push_bulk.rb:13:          .each_slice(batch_size)
./app/views/shops/shop_cards/push_report.pdf.haml:15:            - @user_messages.each_slice(10).to_a.each do |user_messages|
./app/views/shops/shop_cards/push_report.pdf.haml:71:            - @auto_messages_info.each_slice(10).to_a.each do |auto_messages|
./lib/tasks/oneshot/20220526_create_expiration_user_transaction.rake:48:      user_transactions.each_slice(10_000) do |values|
./lib/tasks/shopify_collection_sync.rake:18:        shopify_product_ids.each_slice(10) do |product_ids|
```

### private/protected/public/module_functionが引数を返す

引数に入れて処理するみたいなことをしている箇所はなし

### keyword_initオプションを明示的に指定せずに生成されたStructに対してキーワード引数だけでnewしようとすると警告

spec周りでstructを使用しているが、keyword引数を使用してinitializeしている箇所はなし

### $LOAD_PATH.resolve_feature_pathにloadできない値を渡しても例外が発生しなくなった

使用箇所なし

### refine内でのinclude/prependが非推奨

使用箇所なし

### 一部のdefault gemがbundled gem

- net-ftp 0.1.3
  - 使用箇所なし
- net-imap 0.2.2
  - メール連携周りで使用。
  - ただし、メール連携は現在使用していないので削除すれば良さそう
- net-pop 0.1.1
  - 使用箇所なし
- net-smtp 0.3.1
  - mail gem内で使用されてるかも
  - 対応済みか確認
- matrix 0.4.2
  - 使用箇所なし
- prime 0.1.2
  - 使用箇所なし
- debug 1.4.0
  - 使用箇所なし

### ERB#initialize の第2引数以降が常に警告

使用箇所なし

# タスク

- メール連携機能削除(net/imapを使用しているため)
- mail gemでnet/smtpを使用している箇所があるか確認
  - もし使用しているならどのversionで対応済みかも確認
  - 2.8.0以降であれば対応されていそう。
  - やること -> 2.8.1にあげる

# 動作確認が必要な項目

- EventRepository受信
- メールが送れるかどうか
