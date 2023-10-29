# OrderedOptions

Hashのkey-valueの値をあたかもattributeのように使用することができる

```ruby
h = ActiveSupport::OrderedOptions.new
h.boy = 'John'
h.girl = 'Rose'
h.boy # => 'John'
b.girl # => 'Rose'
b.age # => nil
b.age! # => raise KeyError
```

# InheritableOptions

他のhashからOrderedOptionsを作成することができるクラス

```ruby
h = ActiveSupport::InheritableOptions.new({ girl: 'Mary', boy: 'John' })
h.girl # => 'Mary'
h.boy  # => 'John'
```

TODO: rails7からdigメソッドが増えているので、PRがmergeされ次第修正
