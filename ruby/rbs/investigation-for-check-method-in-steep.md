# Steep

Steepfileにおいて、 `check` メソッドが記述したdirectory以外のファイルも静的型解析を行なっているように見える

```ruby
target :app do
  signature "sig"

  check "app/models"
  check "shopify/app/models"
end
```

この場合に、 `.gem_rbs_collection/` や `vendor/bundle/` 配下も解析してしまっている気がする
コードを読んで正しく動作しているかを確認する

例↓
```shell
$ steep check

.gem_rbs_collection/concurrent-ruby/1.1/utility/processor_counter.rbs:2:2: [error] Non-overloading method definition of `physical_processor_count` in `::Concurrent` cannot be duplicated
│ Diagnostic ID: RBS::DuplicatedMethodDefinition
│
└   def self.physical_processor_count: () -> Integer
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

vendor/bundle/ruby/3.0.0/gems/steep-1.5.3/sig/steep/ast/annotation/collection.rbs:9:29: [error] Cannot find type `RBS::Resolver::context`
│ Diagnostic ID: RBS::UnknownTypeName
│
└         attr_reader context: RBS::Resolver::context
                               ~~~~~~~~~~~~~~~~~~~~~~
```
