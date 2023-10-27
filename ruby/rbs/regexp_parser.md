```ruby.rbs
class Regexp::Parser
  class Error < StandardError
  end
  class ParserError < Error
  end
  class UnknownTokenTypeError < ParserError
    def initialize: (Regexp::Token::token_type type, Regexp::Token token) -> void
  end
  class UnknownTokenError < ParserError
    def initialize: (Regexp::Token::token_type type, Regexp::Token token) -> void
  end

  def parse: (Regexp | String input, String? syntax, ?options: Integer?) ?{ (Regexp::Expression::Root) -> untyped } -> untyped
  def self.parse: (Regexp | String input, ?String? syntax, ?options: Integer?) ?{ (Regexp::Expression::Root) -> untyped } -> Regexp::Expression::Root
end

module Regexp::Expression
  class Root < Subexpression[Root]
  end

  class Subexpression[T < Base] < Base
    include Enumerable[T]

    attr_accessor expressions: Array[T]

    type traversal_event = :enter | :visit | :exit
    def traverse: (?bool include_self) -> Enumerator[T, untyped]
                | (?bool include_self) { (traversal_event, (self | T), Integer) -> untyped } -> self
    alias walk traverse

    def each_expression: (?bool include_self) -> Enumerator[T, untyped]
                        | (?bool include_self) { ((self | T), Integer) -> void } -> self

    def each: () { (T) -> void } -> void
            | () -> Enumerator[T, void]

    def flat_map: (?bool include_self) -> Array[T]
                | (?bool include_self) -> Enumerator[T, Array[T]]
                | (?bool include_self) { (T) -> untyped } -> Array[T]
  end

  class Base
    include Shared
  end

  module Shared
    attr_accessor type: Symbol
  end
end

class Regexp
  TOKEN_KEYS: Array[Symbol]

  class Token
    type token_type = :anchor | :assertion | :group | :backref | :conditional | :escape | :free_space | :keep | :literal | :meta | :posixclass | :nonposixclass | :property | :nonproperty | :quantifier | :set | :type
    attr_reader type: token_type
    attr_reader token: Symbol
    attr_reader text: String
    attr_reader ts: Integer
    attr_reader te: Integer
    attr_reader level: Integer
    attr_reader set_level: Integer
    attr_reader conditional_level: Integer

    attr_accessor previout: instance
    attr_accessor next: instance

    def initialize: (Symbol type, Symbol token, String text, Integer ts, Integer te, Integer level, Integer set_level, Integer conditional_level) -> void

    def offset: () -> [Integer, Integer]
    def length: () -> Integer
  end
end
```


```ruby
# Write Ruby code to test the RBS.
# It is type checked by `steep check` command.

require "regexp_parser"

regex = /a?(b+(c)d)*(?<name>[0-9]+)/i
Regexp::Parser.parse(regex, 'ruby/2.1') do |token|
  puts token.inspect
end
Regexp::Parser.parse(
  'a+ # Recognizes a and A...',
  options: ::Regexp::EXTENDED | ::Regexp::IGNORECASE
)
```

```ruby.rbs
class Regexp::Parser
  def parse: (Regexp | String input, ?String? syntax, ?options: Integer?) { (Regexp::Expression::Root) -> untyped } -> untyped
           | (Regexp | String input, ?String? syntax, ?options: Integer?) -> Regexp::Expression::Root
  def self.parse: (Regexp | String input, ?String? syntax, ?options: Integer?) { (Regexp::Expression::Root) -> untyped } -> untyped
                | (Regexp | String input, ?String? syntax, ?options: Integer?) -> Regexp::Expression::Root
end

module Regexp::Expression
  class Root < Subexpression
  end

  class Subexpression < Base
  end

  class Base
  end
end
```