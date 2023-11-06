```ruby
def destroy
  query = <<~SQL
          query FETCH_ORDERS_WITH_DISCOUNT($query: String!) {
            orders(first: 1, query: $query) {
              nodes {
                id
              }
            }
          }
        SQL

  unless shop_card_point_setting.enable_point_discount?
    api_error(status: :bad_request, message: 'invalid request')
    return
  end

  point_allocated_discount = user.shopify_point_allocated_discount
  if point_allocated_discount.blank?
    api_error(status: :bad_request, message: 'invalid request')
    return
  end

  client = Shopify::Api::Client.new(shop_domain: shop_authentication.shopify_domain, access_token: shop_authentication.shopify_token)
  response = client.query(query: query, variables: { query: "discount_code:#{params[:coupon_code]}" })

  if response.data.dig(:nodes).present?
    api_error(status: :bad_request, message: 'invalid request')
    return
  end

  Shopify::DeleteShopifyDiscountWorker.new.perform(
    "gid://shopify/DiscountCodeNode/#{point_allocated_discount.identifier}",
    shop_authentication.id,
  )

  ActiveRecord::Base.transaction do
    user_transaction = point_allocated_discount.user_transaction
    user_transaction.destroy!
    point_allocated_discount.destroy!

    user_stamp = user.user_cards.find_by(shop_card: external_site.shop_card).current_user_stamp
    user_stamp.update!(stamps: user_stamp.stamps + (-user_transaction.points))
  end

  render json: {
    current_points: current_valid_points,
  }, status: :ok
end
```