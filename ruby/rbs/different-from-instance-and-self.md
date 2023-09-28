# instanceとselfの違い

## instance

クラスのinstance型を返す。また、 `class` はクラスのsingleton型を返す。

https://github.com/protocolbuffers/protobuf/blob/bef5b759f1fa511e3c08380a4e8635832d5d39fd/ruby/lib/google/protobuf/ffi/message.rb#L217

```rbs
def self.decode_json: (String json, Hash[untyped, untyped] options) -> instance
```

## self

receiverの型を返す。
